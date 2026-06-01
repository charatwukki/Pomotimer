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

fn main() {
    // let duration = humantime::parse_duration("50s").unwrap();
    // println!("{}", duration.as_millis());
    let args = Args::parse();

    match args.command {
        Commands::Run { study, rest } => {
            use humantime::parse_duration;
            send_notification(
                "Study Finished",
                &format!("Your study timer of {} is finished", study),
            );
            play_finish();
            timer(parse_duration(&study).unwrap().as_secs());
            send_notification(
                "Break Finished",
                &format!("Your rest timer of {} is finished", rest),
            );
            timer(parse_duration(&rest).unwrap().as_secs());
            play_end();
        }
        Commands::Connect { .. } => {
            println!("Sorry I haven't implemented this yet. Coming soon tho!!")
        }
    }
}
