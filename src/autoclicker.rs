use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use crate::mousecontroller::MouseController;
use rdev::{Button as RdevButton, Event, EventType, Key, listen};

pub struct AutoClicker<M: MouseController> {
    mouse: M,
    click_interval_secs: u64,
    running: Arc<AtomicBool>,
}

impl<M: MouseController> AutoClicker<M> {
    pub fn new(mouse: M, click_interval_secs: u64) -> Self {
        Self {
            mouse,
            click_interval_secs,
            running: Arc::new(AtomicBool::new(true)),
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

    #[allow(dead_code)]
    pub fn get_running(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }

    pub fn do_one_click(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.mouse.move_mouse(x, y)?;
        self.mouse.click()?;
        Ok(())
    }

    pub fn run_click_loop(&mut self, x: i32, y: i32) {
        while self.running.load(Ordering::SeqCst) {
            if let Err(e) = self.do_one_click(x, y) {
                eprintln!("Click failed: {}", e);
            } else {
                println!("Clicked at point ({}, {})", x, y);
            }
            for _ in 0..self.click_interval_secs {
                if !self.running.load(Ordering::SeqCst) {
                    break;
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    pub fn run(&mut self) {
        let (click_tx, click_rx) = mpsc::channel();
        let running_flag = Arc::clone(&self.running);

        self.listen_events(click_tx, Arc::clone(&running_flag));

        println!("Please left-click to set the click point.");
        println!("Press ESC to quit the program at any time.");

        match click_rx.recv() {
            Ok(()) => {
                let (x, y) = self.mouse.location().expect("Failed to get mouse position");

                println!("Clicked at point ({}, {}).", x, y);

                self.run_click_loop(x, y);

                println!("Program terminated by user request (ESC).");
            }
            Err(e) => eprintln!("Error while waiting for click: {}", e),
        }
    }
}
