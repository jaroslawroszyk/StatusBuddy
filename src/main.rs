mod autoclicker;
mod config;
mod mousecontroller;

use autoclicker::AutoClicker;
use config::Config;
use mousecontroller::{EnigoMouse};
use std::env;

fn main() {
    let config = Config::parse_args(env::args()).expect("Failed to parse config");
    println!("Interval time is {}", config.interval_time);

    let mouse = EnigoMouse::new().expect("Failed to create mouse controller");

    let mut auto_clicker = AutoClicker::new(mouse, config.interval_time);
    auto_clicker.run();
}
