use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::{thread, io};
use std::num::ParseIntError;

use rust_drone_follow::hat_follower_settings::HatFollowerSettings;
use rust_drone_follow::hat_file_reader::read_file;
use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::naive_detector::NaiveDetector;
use rust_drone_follow::controllers::mock_controller::MockController;

use crate::parrot::parrot_controller::ParrotController;
use crate::kalman_filter::KalmanFilter;

use parrot_ar_drone::NavDataValue;

pub fn run_follow_test() {

    // let mut controller = VirtualController::new(10.0, StandStill::new(), false);
    let system_time = SystemTime::now();
    let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut settings = HatFollowerSettings::debug();

    settings.save_to_file = Some(format!("video_{}.mp4", seconds));
    settings.save_commands = Some(format!("commands_{}.txt", seconds));
    settings.center_threshold = 10.0;
    settings.min_change = 0.00;

    let hat_file = "bayern.hat";

    follow_test(hat_file, settings);
    //run_noui_follow(hat_file, settings);
}

fn follow_test(filename: &str, settings: HatFollowerSettings) {
    let (vid, hat) = read_file(filename);
    let mut hf = HatFollower::new(
        NaiveDetector::new(hat),
        MockController::new(vid.as_str(), 640, 368),
        KalmanFilter::new(1.0, 1.1, 1.0),
        settings,
        None,
    );

    hf.run();
}

pub fn drone_test() {
    println!("Start!");
    let mut drone = parrot_ar_drone::Drone::new();

    println!("Instantiating done!");
    drone.startup().unwrap();
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

pub fn run_noui_follow(filename: &str, settings: HatFollowerSettings) {
    let (sx, rx) = std::sync::mpsc::channel();
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
                sx.send(0).unwrap();
                break;
            }
            Ok(_) => {

            }
        }
    }

    handle.join().unwrap();
}

fn read_int() -> Result<i32, ParseIntError> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<i32>()
}
