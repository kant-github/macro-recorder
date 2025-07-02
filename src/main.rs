use chrono::Utc;
use csv::Writer;
use rdev::{Event, EventType, Key, listen};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() {
    println!("What should we call this recording session :?");
    let mut session_name: String = String::new();
    std::io::stdin().read_line(&mut session_name).unwrap();
    
}
