#[macro_use]
extern crate rulinalg;
extern crate linearkalman;

mod parrot;
mod kalman_filter;
mod simulation;
mod ui;
mod utils;

use iced::{Settings, Sandbox};
use ui::tour::Tour;

fn main() {

    println!("Starting up the UI");
    let mut iced_settings = Settings::<()>::default();
    iced_settings.window.size = (560, 700);
    Tour::run(iced_settings);
}

