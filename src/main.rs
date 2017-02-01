extern crate notify_rust;

use notify_rust::Notification;

use std::time::Duration;
use std::thread;

// const POMODORO_SECONDS: u64 = 60*24;
const POMODORO_SECONDS: u64 = 3;
const SHORT_PAUSE_SECONDS: u64 = 60*5;
const LONG_PAUSE_SECONDS: u64 = 60*20;

fn blocking_notification(summary: &str, body: &str) {
  Notification::new()
    .summary(summary)
    .body(body)
    .show().unwrap()
    .wait_for_action(|_action| {
      println!("Notification was closed");
    });  
}

fn main() {
  let pomodoro = Duration::new(POMODORO_SECONDS, 0);
  let short_pause = Duration::new(SHORT_PAUSE_SECONDS, 0);
  let long_pause = Duration::new(LONG_PAUSE_SECONDS, 0);

  let mut count = 0;
  loop {
    blocking_notification("New Pomodoro", "");
    println!("Starting new pomodoro");
    thread::sleep(pomodoro);

    println!("Pomodoro done");
    count += 1;

    let duration = if count % 4 == 0 {
      println!("ready for a long pause");
      blocking_notification("Good job, take a long pause now", "");
      long_pause
    } else {
      println!("ready for a short pause");
      blocking_notification("Short Pause", "");
      short_pause
    };
    thread::sleep(duration);
  }
}
