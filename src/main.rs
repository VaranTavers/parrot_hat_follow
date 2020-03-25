use std::{thread, io};

use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::naive_detector::NaiveDetector;
use rust_drone_follow::hat_file_reader::read_file;
use rust_drone_follow::filters::no_filter::NoFilter;

use parrot_ar_drone::NavDataValue;

mod parrot_controller;
mod mock_printer_controller;

use parrot_controller::ParrotController;
use std::time::Duration;
use mock_printer_controller::MockPrinterController;
use rust_drone_follow::hat_follower_settings::HatFollowerSettings;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
fn read_int() -> i32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    parse_input!(input_line.trim(), i32)
}

fn main() {
    // drone_test();
    follow_test();
    // run_follow();
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

fn follow_test() {
    let (vid, hat) = read_file("benti.hat");
    let mut settings = HatFollowerSettings::debug();
    settings.min_change = 0.3;
    let mut hf = HatFollower::new(
        NaiveDetector::new(hat),
        MockPrinterController::new(vid.as_str(), 640, 368),
        NoFilter::new(),
        settings,
        None,
    );

    hf.run();
}

fn run_follow() {
    let (mut sx, rx) = std::sync::mpsc::channel();
    let (_, hat) = read_file("benti.hat");

    let handle = thread::spawn(|| {
        let mut hf = HatFollower::new(
            NaiveDetector::new(hat),
            ParrotController::new(),
            NoFilter::new(),
            HatFollowerSettings::debug(),
            Some(rx)
        );

        hf.run();
    });

    loop {
        let i = read_int();
        if i == 0 {
            sx.send(0);
            break;
        }
    }

    handle.join().unwrap();
}
