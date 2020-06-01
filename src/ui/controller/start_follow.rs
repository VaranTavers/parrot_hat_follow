use std::{fs, thread};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use rust_drone_follow::utils::hat_file_reader::read_file;
use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::NaiveDetector;

use crate::kalman_filter::KalmanFilter;

use crate::simulation::virtual_controller::VirtualController;
use crate::simulation::movetactics::move_squares::MoveSquares;
use crate::simulation::windtactics::periodic_wind::PeriodicWind;

use crate::utils::file_readers::{read_follow_file, read_kalman_file};

pub fn start_follow() -> (JoinHandle<()>, Sender<i32>) {
    let settings = read_follow_file("config.follow");
    let (sigma0, sigma_gain, est_v_loss) = read_kalman_file("config.kalman");

    let (sx, rx) = std::sync::mpsc::channel();
    let (_, hat) = read_file("config.hat");
    let join_handle = thread::spawn(move || {
        let mut hf = HatFollower::new(
            NaiveDetector::new(hat),
            VirtualController::new(20.0, 1, 0.01,
                                   MoveSquares::new(0.7, 500),
                                   // StandStill::new(),
                                   // NoWind::new(), false),
                                   PeriodicWind::new_polar(3.0, 3.81, 150, 2000), false),
            // ParrotController::new(300, true),
            KalmanFilter::new(sigma0, sigma_gain, est_v_loss),
            settings,
            Some(rx)
        );
        hf.run();
    });
    (join_handle, sx)
}