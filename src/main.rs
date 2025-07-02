use chrono::Utc;
use csv::Writer;
use rdev::{Event, EventType, Key, listen};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, serde::Serialize)]
struct InputEvent {
    timestamp: u128,
    event_type: String,
    key_or_button: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
}
fn main() {
    println!("What should we call this recording session :?");
    let mut session_name: String = String::new();
    std::io::stdin().read_line(&mut session_name).unwrap();

    println!("session name is : {}", session_name);

    println!("Got it, starting in..");
    for i in (1..=5).rev() {
        println!("...{}", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Recording started! press ESC to cancel");

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let start_time = Instant::now();

    
}
