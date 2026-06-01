use core::time;

use clap::{Parser, Subcommand};
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a simple Pomodoro timer.
    Run {
        #[arg(default_value = "50m")]
        study: String,
        #[arg(default_value = "10m")]
        rest: String,
    },
    /// Connect to a peer (not implemented)
    Connect {
        #[arg(default_value = "")]
        name: String,
    },
}
fn play_finish() {
    use std::io::BufReader;
    use std::io::Cursor;
    let mut sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    sink_handle.log_on_drop(false);
    let cursor = Cursor::new(include_bytes!("../assets/sfx/POMODORO-FINISH.wav").as_ref());
    let file = BufReader::new(cursor);
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
    player.set_volume(0.2);
    player.sleep_until_end();
}

fn play_end() {
    use std::io::BufReader;
    use std::io::Cursor;
    let mut sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    sink_handle.log_on_drop(false);
    let cursor = Cursor::new(include_bytes!("../assets/sfx/POMODORO-BREAK-END.wav").as_ref());
    let file = BufReader::new(cursor);
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
    player.set_volume(0.2);
    player.sleep_until_end();
}
fn send_notification(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(5000)
        .show()
        .expect("failed to send notification");
}
fn timer(secs: u64) {
    for i in 0..secs {
        println!("{}/{}", i + 1, secs);
        std::thread::sleep(time::Duration::from_secs(1))
    }
}
fn print_bar(width: u16) {
    use crossterm::execute;
    use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
    use std::io::stdout;
    for i in 0..width.clamp(0, 75) {
        let g = 50 + (i / 2) as u8;
        let base_r = 255 - width as u8 - (i * 2) as u8;
        let r = if width < 10 {
            (base_r as f32 * (width as f32 / 10.0).powf(0.3)).max(0.0) as u8
        } else {
            base_r
        };
        execute!(
            stdout(),
            SetForegroundColor(Color::Rgb { r, g, b: 255 }),
            Print("█"),
            ResetColor
        )
        .unwrap();
    }
    for _ in 0..75 - width {
        execute!(
            stdout(),
            SetForegroundColor(Color::Rgb {
                r: 45,
                g: 45,
                b: 45
            }),
            Print("█"),
            ResetColor
        )
        .unwrap();
    }
    println!("{}", ResetColor);
}
fn print_bar_percent(percentage: f32) {
    print_bar((percentage * (75 as f32)) as u16);
}

fn main() {
    use crossterm::{cursor, execute, terminal};
    use std::io::stdout;
    execute!(stdout(), cursor::SavePosition).unwrap();
    for i in 0..101 {
        execute!(
            stdout(),
            cursor::RestorePosition,
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap();
        print_bar_percent({ i as f32 } / 100.0);
        std::thread::sleep(time::Duration::from_millis(200));
    }

    let args = Args::parse();

    match args.command {
        Commands::Run { study, rest } => {
            use humantime::parse_duration;
            timer(parse_duration(&study).unwrap().as_secs()); // BUG: I need to error handle this
                                                              // but i don't care tbh
            send_notification(
                "Study Finished",
                &format!("Your study timer of {} is finished", study),
            );
            play_finish();
            timer(parse_duration(&rest).unwrap().as_secs());
            send_notification(
                "Break Finished",
                &format!("Your rest timer of {} is finished", rest),
            );
            play_end();
        }
        Commands::Connect { .. } => {
            println!("Sorry I haven't implemented this yet. Coming soon tho!!")
        }
    }
}
