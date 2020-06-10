use std::{fs, thread};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use rust_drone_follow::utils::hat_file_reader::read_file;
use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::NaiveDetector;

use crate::kalman_filter::KalmanFilter;

use crate::utils::file_readers::{read_follow_file, read_kalman_file, read_controller_file};

use crate::simulation::virtual_controller::VirtualController;
use crate::simulation::movetactics::move_squares::MoveSquares;
use crate::simulation::windtactics::periodic_wind::PeriodicWind;
use crate::simulation::windtactics::random_wind::RandomWind;
use crate::simulation::movetactics::stand_still::StandStill;

pub fn start_follow() -> (JoinHandle<()>, Sender<i32>) {
    let settings = read_follow_file("config.follow");
    let (sigma0, sigma_gain, est_v_loss) = read_kalman_file("config.kalman");
    let (p_c_opt, v_c_opt) = read_controller_file("config.controller");

    let (sx, rx) = std::sync::mpsc::channel();
    let (_, hat) = read_file("config.hat");
    let join_handle = match p_c_opt {
        Some(controller) => {
            thread::spawn(move || {
                let mut hf = HatFollower::new(
                    NaiveDetector::new(hat),
                    controller,
                    KalmanFilter::new(sigma0, sigma_gain, est_v_loss),
                    settings,
                    Some(rx)
                );
                hf.run();
            })
        }
        None => {
            if let Some(controller) = v_c_opt {
                thread::spawn(move || {
                    let mut hf = HatFollower::new(
                        NaiveDetector::new(hat),
                        controller,
                        KalmanFilter::new(sigma0, sigma_gain, est_v_loss),
                        settings,
                        Some(rx)
                    );
                    hf.run();
                })
            } else {
                thread::spawn(move || {
                    let mut hf = HatFollower::new(
                        NaiveDetector::new(hat),
                        VirtualController::new(20.0, 1, 0.01, StandStill::new(), PeriodicWind::new_polar(4.1, 0.3, 80, 500), false),
                        KalmanFilter::new(sigma0, sigma_gain, est_v_loss),
                        settings,
                        Some(rx)
                    );
                    hf.run();
                })
            }
        }
    };
    (join_handle, sx)
}