use notify_rust::Notification;
fn main() {
    println!("Hello, world!");
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show()
        .expect("failed to send notification");
}
