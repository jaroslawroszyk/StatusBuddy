use enigo::{Button, Coordinate::Abs, Direction::Click, Enigo, Mouse, Settings};

pub trait MouseController {
    fn location(&self) -> Result<(i32, i32), String>;
    fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), String>;
    fn click(&mut self) -> Result<(), String>;
}

pub struct EnigoMouse {
    enigo: Enigo,
}

impl EnigoMouse {
    pub fn new() -> Result<Self, String> {
        Ok(Self { enigo: Enigo::new(&Settings::default()).map_err(|e| e.to_string())? })
    }
}

impl MouseController for EnigoMouse {
    fn location(&self) -> Result<(i32, i32), String> {
        self.enigo.location().map_err(|e| e.to_string())
    }
    fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), String> {
        let _ = self.enigo.move_mouse(x, y, Abs);
        Ok(())
    }
    fn click(&mut self) -> Result<(), String> {
        let _ = self.enigo.button(Button::Left, Click);
        Ok(())
    }
}
