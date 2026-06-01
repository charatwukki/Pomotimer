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
        // #[arg(short)]
        name: String,
    },
}
fn play_noise(noise: &str) {
    use std::fs::File;
    use std::io::BufReader;
    let mut sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    sink_handle.log_on_drop(false);
    let file = BufReader::new(File::open("assets/sfx/".to_string() + noise).unwrap());
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
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

fn main() {
    // let duration = humantime::parse_duration("50s").unwrap();
    // println!("{}", duration.as_millis());
    let args = Args::parse();

    match args.command {
        Commands::Run { study, rest } => {
            use humantime::parse_duration;
            println!("Hello, world!");
            println!("{:?}, {}", parse_duration(&study).unwrap().as_secs(), rest);
            send_notification("Study Finished", &format!("Your study timer of {} is finished", study));
            play_noise("POMODORO-FINISH.wav");
            send_notification("Break Finished", &format!("Your rest timer of {} is finished", rest));
            play_noise("POMODORO-BREAK-END.wav");
        }
        Commands::Connect { .. } => {}
    }
}
