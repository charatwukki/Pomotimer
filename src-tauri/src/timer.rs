use std::{sync::Mutex, time::Instant};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub fn create_timer(state: tauri::State<Timers>) -> u32 {
    let mut timers = state.timers.lock().unwrap();
    timers.push(Instant::now());
    (timers.len() - 1) as u32
}

#[tauri::command]
#[specta::specta]
pub fn get_timer(state: tauri::State<Timers>, index: u32) -> u32 {
    let timers = state.timers.lock().unwrap();
    timers[index as usize].elapsed().as_secs() as u32
}

pub struct Timers {
    timers: Mutex<Vec<Instant>>,
}
impl Timers {
    pub fn new() -> Timers {
        Timers {
            timers: Mutex::new(vec![Instant::now()]),
        }
    }
}
