#[macro_use]
extern crate rulinalg;
extern crate linearkalman;

use std::{thread, io};
use std::num::ParseIntError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::naive_detector::NaiveDetector;
use rust_drone_follow::hat_file_reader::read_file;
// use rust_drone_follow::filters::no_filter::NoFilter;
// use rust_drone_follow::filters::memory_filter::MemoryFilter;

use parrot_ar_drone::NavDataValue;

mod parrot_controller;
mod kalman_filter;
mod mock_printer_controller;
mod ui;

use parrot_controller::ParrotController;
use mock_printer_controller::MockPrinterController;
use rust_drone_follow::hat_follower_settings::HatFollowerSettings;
use crate::kalman_filter::KalmanFilter;
use iced::{Sandbox, Settings};

fn read_int() -> Result<i32, ParseIntError> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<i32>()
}

fn main() {
    let system_time = SystemTime::now();
    let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut settings = HatFollowerSettings::debug();
    settings.save_to_file = Some(format!("video_{}.mp4", seconds));
    settings.save_commands = Some(format!("commands_{}.txt", seconds));
    settings.center_threshold = 10.0;
    settings.min_change = 0.00;

    let hat_file = "bayern.hat";
    // drone_test();
    // follow_test(hat_file, settings);
    // run_follow(hat_file, settings);

    ui::SettingsEditor::run(Settings::default());
}

fn drone_test() {
    println!("Start!");
    let mut drone = parrot_ar_drone::Drone::new();

    println!("Instantiating done!");
    drone.startup();
    println!("Takeoff in 5");
    thread::sleep(Duration::from_secs(5));
    match drone.get_navdata("demo_battery") {
        Some(NavDataValue::Uint(a)) => { println!("Battery: {}%", a); }
        _ => { println!("Battery status unknown!"); }
    }
    println!("Takeoff");
    drone.takeoff();
    thread::sleep(Duration::from_secs(5));
    drone.mov_up(0.3);
    thread::sleep(Duration::from_secs(2));
    drone.mov_down(0.3);
    println!("Land in 5");
    thread::sleep(Duration::from_secs(2));
    println!("Landing");
    drone.land();
}

fn follow_test(filename: &str, settings: HatFollowerSettings) {
    let (vid, hat) = read_file(filename);
    let mut hf = HatFollower::new(
        NaiveDetector::new(hat),
        MockPrinterController::new(vid.as_str(), 640, 368),
        KalmanFilter::new(1.0, 1.1, 1.0),
        settings,
        None,
    );

    hf.run();
}

fn run_follow(filename: &str, settings: HatFollowerSettings) {
    let (mut sx, rx) = std::sync::mpsc::channel();
    let (_, hat) = read_file(filename);

    let handle = thread::spawn(|| {
        let mut hf = HatFollower::new(
            NaiveDetector::new(hat),
            ParrotController::new(220, true),
            KalmanFilter::new(1.0, 1.1, 1.0),
            settings,
            Some(rx)
        );

        hf.run();
    });

    loop {
        let i = read_int();
        match i {
            Ok(0) | Err(_) => {
                sx.send(0);
                break;
            }
            Ok(_) => {

            }
        }
    }

    handle.join().unwrap();
}
