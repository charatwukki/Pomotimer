use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::sync::Mutex;
struct Dc {
    dc: DiscordIpcClient,
    connected: bool,
}
pub struct Discord {
    dc: Mutex<Option<Dc>>,
}

impl Discord {
    pub fn new() -> Discord {
        Discord {
            dc: Mutex::new(None),
        }
    }
}

impl Drop for Dc {
    fn drop(&mut self) {
        if self.dc.close().is_ok() {
        } else {
            if self.connected {
                println!("warning: Discord connection improperly closed.");
            }
        }
    }
}

#[tauri::command]
#[specta::specta]
/// Set discord status.
/// Name of activity, e.g, maths, science and stuff
/// Time in seconds for completion
/// Pomotype, check bindings for all possible pomotypes (will err in console if invalid)
pub async fn set_status(
    state: tauri::State<'_, Discord>,
    pomotype: PomoType,
) -> Result<(), String> {
    let mut lock = state.dc.lock().await;
    let client = lock.get_or_insert_with(|| Dc {
        dc: DiscordIpcClient::new("1503690275783184454"),
        connected: false,
    });
    if client.dc.connect().is_ok() {
        if client.dc.set_activity(pomo_activity(pomotype)).is_ok() {
            client.connected = true;
        } else {
            eprintln!("warning: Discord rich presence not established.")
        }
    } else {
        eprintln!("warning: Can't find discord client")
    }
    Ok(())
}
#[derive(Type, Deserialize, Serialize)]
pub struct StudyArgs {
    seconds: u32,
    name: String,
}
#[derive(Type, Deserialize, Serialize)]
pub struct RestArgs {
    seconds: u32,
    name: String,
}

#[derive(Type, Deserialize, Serialize)]
pub enum PomoType {
    Study(StudyArgs),
    Rest(RestArgs),
    AFK,
}

fn pomo_activity(pomotype: PomoType) -> discord_rich_presence::activity::Activity<'static> {
    use discord_rich_presence::activity::{
        Activity, ActivityType, Assets, Button, StatusDisplayType, Timestamps,
    };
    use std::time::{SystemTime, UNIX_EPOCH};

    let dnow = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut act = Activity::new() // TODO: Implement secrets/party when i make multiplayer.
        .name("Pomodoro")
        .details_url("https://example.com/details") // TODO: Point this to readme
        .activity_type(ActivityType::Competing)
        .status_display_type(StatusDisplayType::Name)
        .buttons(vec![
            Button::new("Source Code", "https://github.com/charatwukki/Pomotimer"),
            // Button::new("Visit Site", "https://example.com"),
        ]);
    match pomotype {
        PomoType::Rest(args) => {
            act = act
                .details("Resting zzz...")
                .timestamps(
                    Timestamps::new()
                        .start(dnow)
                        .end(dnow + args.seconds as i64),
                )
                .assets(
                    Assets::new()
                        .large_image("todo")
                        .large_text("Pomodoro Timer")
                        .small_image("rest")
                        .small_text("Resting"),
                )
                .state(format!("Doing {}", args.name)) // TODO: fix on empty
                .state_url("https://example.com/state");
        }
        PomoType::Study(args) => {
            act = act
                .details("Working Hard!")
                .timestamps(
                    Timestamps::new()
                        .start(dnow)
                        .end(dnow + args.seconds as i64),
                )
                .assets(
                    Assets::new()
                        .large_image("todo")
                        .large_text("Pomodoro Timer")
                        .small_image("study")
                        .small_text("Studying"),
                )
                .state(format!("Doing {}", args.name)) // TODO: fix on empty
                .state_url("https://example.com/state");
        }
        _ => {}
    };
    act
}
