mod autoclicker;
mod config;

use autoclicker::AutoClicker;
use config::Config;
use std::env;

fn main() {
    let config = Config::parse_args(env::args());
    println!("Interval time is {}", config.clone().unwrap().interval_time);

    let mut auto_clicker = AutoClicker::new(config.unwrap().interval_time);
    auto_clicker.run();
}
