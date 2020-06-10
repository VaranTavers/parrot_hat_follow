use rust_drone_follow::HatFollowerSettings;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::parrot::parrot_controller::ParrotController;
use crate::simulation::virtual_controller::VirtualController;
use crate::simulation::movetactics::move_squares::MoveSquares;
use crate::simulation::windtactics::periodic_wind::PeriodicWind;
use crate::simulation::windtactics::random_wind::RandomWind;

pub fn read_follow_file(filename: &str) -> HatFollowerSettings {
    let follower_content = fs::read_to_string(filename)
        .expect("Something went wrong reading the config.follower file");
    let follower_args: Vec<&str> = follower_content.split(' ').collect::<Vec<&str>>();

    let mut settings = match follower_args[0] {
        "Video" => HatFollowerSettings::new(),
        "Silent" => HatFollowerSettings::silent(),
        _ => {
            let mut debug = HatFollowerSettings::debug();
            let system_time = SystemTime::now();
            let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
            debug.save_to_file = Some(format!("video_{}.mp4", seconds));
            debug.save_commands = Some(format!("commands_{}.txt", seconds));
            debug
        }
    };

    settings.turn_range = 0.01;
    settings.center_threshold = match follower_args[1].trim().parse::<f64>() {
        Ok(ct) => ct,
        _ => 10.0
    };
    settings.min_change = match follower_args[2].trim().parse::<f64>() {
        Ok(mc) => mc,
        _ => 0.1
    };

    settings
}

pub fn read_kalman_file(filename: &str) -> (f64, f64, f64) {
    let kalman_content = fs::read_to_string(filename)
        .expect("Something went wrong reading config.kalman the file");
    let kalman_args: Vec<&str> = kalman_content.split(' ').collect::<Vec<&str>>();

    let sigma0 = match kalman_args[0].trim().parse::<f64>() {
        Ok(mc) => mc,
        _ => 1.0
    };
    let sigma_gain = match kalman_args[1].trim().parse::<f64>() {
        Ok(mc) => mc,
        _ => 1.1
    };
    let est_v_loss = match kalman_args[2].trim().parse::<f64>() {
        Ok(mc) => mc,
        _ => 1.0
    };

    (sigma0, sigma_gain, est_v_loss)
}

pub fn read_controller_file(filename: &str) -> (Option<ParrotController>, Option<VirtualController<MoveSquares, RandomWind>>) {
    let kalman_content = fs::read_to_string(filename)
        .expect("Something went wrong reading controller.kalman the file");
    let kalman_args: Vec<&str> = kalman_content.split('\n').collect::<Vec<&str>>();

    match kalman_args[0] {
        "ParrotController" => {
            (Some(ParrotController::new(300, true)), None)
        }
        _ => {

            (None, Some(VirtualController::new(20.0, 1, 0.01,
                MoveSquares::new(0.7, 500),
                RandomWind::new_polar(3.0, 150, 2000), false,
            )))
        }
    }
}
