
use std::sync::atomic::{AtomicUsize, Ordering};

use status_buddy::mousecontroller::MouseController;


pub struct MockMouseController {
    pub location_calls: AtomicUsize,
    pub move_calls: AtomicUsize,
    pub click_calls: AtomicUsize,
}

impl MockMouseController {
    pub fn new() -> Self {
        Self {
            location_calls: AtomicUsize::new(0),
            move_calls: AtomicUsize::new(0),
            click_calls: AtomicUsize::new(0),
        }
    }
}

impl MouseController for MockMouseController {
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
