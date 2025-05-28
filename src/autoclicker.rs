use enigo::{Button, Coordinate::Abs, Direction::Click, Enigo, Mouse, Settings};
use rdev::{Button as RdevButton, Event, EventType, Key, listen};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

pub struct AutoClicker {
    enigo: Enigo,
    click_interval_secs: u64,
    running: Arc<AtomicBool>,
}

impl AutoClicker {
    pub fn new(click_interval_secs: u64) -> Self {
        let enigo = Enigo::new(&Settings::default()).expect("Failed to initialize Enigo");
        let running = Arc::new(AtomicBool::new(true));

        Self {
            enigo,
            click_interval_secs,
            running,
        }
    }

    fn listen_events(&self, click_tx: mpsc::Sender<()>, running_flag: Arc<AtomicBool>) {
        thread::spawn(move || {
            listen(move |event: Event| match event.event_type {
                EventType::ButtonPress(btn) if btn == RdevButton::Left => {
                    let _ = click_tx.send(());
                }
                EventType::KeyPress(key) if key == Key::Escape => {
                    running_flag.store(false, Ordering::SeqCst);
                }
                _ => {}
            })
            .expect("Failed to start listening for events");
        });
    }

    pub fn run(&mut self) {
        let (click_tx, click_rx) = mpsc::channel();
        let running_flag = Arc::clone(&self.running);

        self.listen_events(click_tx, Arc::clone(&running_flag));

        println!("Please left-click to set the click point.");
        println!("Press ESC to quit the program at any time.");

        match click_rx.recv() {
            Ok(()) => {
                let (x, y) = self.enigo.location().expect("Failed to get mouse position");

                println!("Clicked at point ({}, {}). Auto-clicker starting...", x, y);

                while self.running.load(Ordering::SeqCst) {
                    self.enigo
                        .move_mouse(x, y, Abs)
                        .expect("Failed to move mouse");
                    self.enigo
                        .button(Button::Left, Click)
                        .expect("Failed to click mouse");

                    println!("Clicked at point ({}, {})", x, y);

                    for _ in 0..self.click_interval_secs {
                        if !self.running.load(Ordering::SeqCst) {
                            break;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                }

                println!("Program terminated by user request (ESC).");
            }
            Err(e) => eprintln!("Error while waiting for click: {}", e),
        }
    }
}
