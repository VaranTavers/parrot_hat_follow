use rust_drone_follow::traits::Controller;
use parrot_ar_drone::Drone;

use std::thread;
use std::time::Duration;

pub struct ParrotController {
    drone: Option<Drone>,
}

impl ParrotController {
    pub fn new() -> ParrotController {
        ParrotController {
            drone: Some(Drone::new()),
        }
    }
}

impl Controller for ParrotController {
    fn init(&mut self) {
        let mut drone = self.drone.take().unwrap();
        match drone.startup() {
            Ok(()) => {
                thread::sleep(Duration::from_secs(2));
                drone.trim();
                drone.use_ground_cam();
            }
            Err(s) => {
                panic!(s);
            }
        }
        self.drone.replace(drone);
    }

    fn shutdown(&mut self) {
        self.drone.take().unwrap();
    }

    fn takeoff(&mut self) {
        let mut drone = self.drone.take().unwrap();
        drone.takeoff();
        drone.mov_up(1.0);
        thread::sleep(Duration::from_secs(2));
        drone.stop();
        self.drone.replace(drone);
    }

    fn land(&mut self) {
        let mut drone = self.drone.take().unwrap();
        drone.land();
        self.drone.replace(drone);
    }

    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64) {
        println!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right);
        let mut drone = self.drone.take().unwrap();
        drone.mov(
            left_right as f32,
            back_front as f32,
            down_up as f32,
            turn_left_right as f32
        );
        self.drone.replace(drone);
    }

    fn stop(&mut self) {
        let mut drone = self.drone.take().unwrap();
        drone.stop();
        self.drone.replace(drone);
    }

    fn get_video_height(&self) -> usize {
        368
    }

    fn get_video_width(&self) -> usize {
        640
    }

    fn get_opencv_url(&self) -> String {
        String::from("tcp://192.168.1.1:5555")
    }

    fn get_kv(&self) -> f64 {
        // NEEDS TESTING
        0.01
    }

    fn get_ka(&self) -> f64 {
        // NEEDS TESTING
        0.01
    }
}