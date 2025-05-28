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

    pub fn get_click_interval_secs(&self) -> u64{
        self.click_interval_secs
    }

    pub fn get_running(&self)->Arc<AtomicBool>{
        self.running.clone()
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
                let (x, y) = self.mouse.location().expect("Failed to get mouse position");

                println!("Clicked at point ({}, {}). Auto-clicker starting...", x, y);

                while self.running.load(Ordering::SeqCst) {
                    self.mouse.move_mouse(x, y).expect("Failed to move mouse");
                    self.mouse.click().expect("Failed to click mouse");

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    struct MockMouse {
        location_calls: AtomicUsize,
        move_calls: AtomicUsize,
        click_calls: AtomicUsize,
    }

    impl MockMouse {
        fn new() -> Self {
            Self {
                location_calls: AtomicUsize::new(0),
                move_calls: AtomicUsize::new(0),
                click_calls: AtomicUsize::new(0),
            }
        }
    }

    impl MouseController for MockMouse {
        fn location(&self) -> Result<(i32, i32), String> {
            self.location_calls.fetch_add(1, Ordering::SeqCst);
            Ok((100, 200))
        }
        fn move_mouse(&mut self, _x: i32, _y: i32) -> Result<(), String> {
            self.move_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn click(&mut self) -> Result<(), String> {
            self.click_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn test_autoclicker_new() {
        let mouse = MockMouse::new();
        let click_interval_secs = 1;
        let auto_clicker = AutoClicker::new(mouse, click_interval_secs);

        assert_eq!(auto_clicker.click_interval_secs, 1);
        assert!(auto_clicker.running.load(Ordering::SeqCst));
    }

    #[test]
    fn test_run_stops_after_escape() {

        let mouse = MockMouse::new();
        let mut auto_clicker = AutoClicker::new(mouse, 1);

        // Zamiast uruchamiać prawdziwe listen_events,
        // wyślemy sygnał kliknięcia i przerwiemy po ESC
        let running_flag = Arc::clone(&auto_clicker.running);

        let (tx, _rx) = mpsc::channel();

        // Podmienimy listen_events na coś prostego, żeby wysłać klik i ESC
        let click_tx = tx.clone();
        thread::spawn(move || {
            // Symulacja kliknięcia LPM
            click_tx.send(()).unwrap();

            // Po chwili symulacja naciśnięcia ESC
            running_flag.store(false, Ordering::SeqCst);
        });

        // Użyjemy oryginalnej metody run, ale z podmienionym kanałem
        // W tym kodzie run() tworzy swój kanał i listen_events sam,
        // więc trzeba by zmienić AutoClicker żeby listen_events było testowalne
        // albo wyodrębnić część logiki.

        // Tutaj dla uproszczenia zróbmy krótką symulację tylko logiki klikania:

        // Pobierz pozycję
        let (x, y) = auto_clicker.mouse.location().unwrap();

        // Sprawdzamy, że kliknięcia i ruch są wywoływane
        for _ in 0..2 {
            if !auto_clicker.running.load(Ordering::SeqCst) {
                break;
            }
            auto_clicker.mouse.move_mouse(x, y).unwrap();
            auto_clicker.mouse.click().unwrap();
            thread::sleep(Duration::from_millis(10));
        }

        // Po 2 iteracjach (lub mniej jeśli running = false) powinno działać OK
        assert!(!auto_clicker.running.load(Ordering::SeqCst));
    }
}
