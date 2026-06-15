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
    name: &str,
    seconds: u32,
    pomotype: PomoType,
) -> Result<(), String> {
    let mut lock = state.dc.lock().await;
    let client = lock.get_or_insert_with(|| Dc {
        dc: DiscordIpcClient::new("1503690275783184454"),
        connected: false,
    });
    if client.dc.connect().is_ok() {
        if client
            .dc
            .set_activity(pomo_activity(name, seconds, pomotype))
            .is_ok()
        {
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
pub enum PomoType {
    Study,
    Rest,
    AFK,
}

fn pomo_activity(
    name: &str,
    seconds: u32,
    pomotype: PomoType,
) -> discord_rich_presence::activity::Activity<'static> {
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
        .state(format!("Doing {}", name)) // TODO: fix on empty
        .state_url("https://example.com/state")
        .activity_type(ActivityType::Competing)
        .status_display_type(StatusDisplayType::Name)
        .timestamps(Timestamps::new().start(dnow).end(dnow + seconds as i64))
        .buttons(vec![
            Button::new("Source Code", "https://github.com/charatwukki/Pomotimer"),
            // Button::new("Visit Site", "https://example.com"),
        ]);
    match pomotype {
        PomoType::Rest => {
            act = act.details("Resting zzz...").assets(
                Assets::new()
                    .large_image("todo")
                    .large_text("Pomodoro Timer")
                    .small_image("rest")
                    .small_text("Resting"),
            );
        }
        PomoType::Study => {
            act = act.details("Working Hard!").assets(
                Assets::new()
                    .large_image("todo")
                    .large_text("Pomodoro Timer")
                    .small_image("study")
                    .small_text("Studying"),
            );
        }
        _ => {}
    };
    act
}
