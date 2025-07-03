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
    let session_name = session_name.trim();

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

    thread::spawn(move || {
        listen(move |event: Event| {
            if !running_clone.load(Ordering::SeqCst) {
                return;
            }

            let mut events = events_clone.lock().unwrap();
            let timestamp = start_time.elapsed().as_millis();

            // pattern matching ----------------------------------- >
            let (event_type, key_or_button, x, y) = match event.event_type {
                EventType::KeyPress(key) => (
                    "key_down".to_string(),
                    Some(format!("{:?}", key)),
                    None,
                    None,
                ),

                EventType::KeyRelease(key) => {
                    ("key_up".to_string(), Some(format!("{:?}", key)), None, None)
                }

                EventType::ButtonPress(button) => (
                    "mouse_down".to_string(),
                    Some(format!("{:?}", button)),
                    None,
                    None,
                ),

                EventType::ButtonRelease(button) => (
                    "mouse_up".to_string(),
                    Some(format!("{:?}", button)),
                    None,
                    None,
                ),

                EventType::MouseMove { x, y } => ("mouse_move".to_string(), None, Some(x), Some(y)),
                _ => return,
            };

            let maybe_key = match event.event_type {
                EventType::KeyPress(k) => Some(k),
                _ => None,
            };

            match maybe_key {
                Some(Key::Escape) => {
                    println!("ESC pressed stopping recording...");
                    running_clone.store(false, Ordering::SeqCst);
                    return;
                }
                _ => {}
            }

            events.push(InputEvent {
                timestamp,
                event_type,
                key_or_button,
                x,
                y,
            });
        })
        .unwrap()
    });

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    let events = events.lock().unwrap();
    let filename = format!("{}_{}.csv", session_name, Utc::now().timestamp());

    let mut wtr = Writer::from_path(&filename).unwrap();
    for event in events.iter() {
        wtr.serialize(event).unwrap();
    }
    wtr.flush().unwrap();

    println!("🎉 Recording saved to {}", filename);
}
