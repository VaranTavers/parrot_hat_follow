use std::thread;
use std::time::Duration;
use std::mem;

use rust_drone_follow::traits::Controller;
use rust_drone_follow::text_exporter::TextExporter;
use parrot_ar_drone::{Drone, NavDataValue};

pub struct ParrotController {
    print_debug: bool,
    flight_height: i32,
    drone: Option<Drone>,
    te: TextExporter,
}

impl ParrotController {
    pub fn new(flight_height: i32, debug: bool) -> ParrotController {
        ParrotController {
            flight_height,
            print_debug: debug,
            drone: Some(Drone::new()),
            te: TextExporter::new(),
        }
    }

    pub fn get_current_flight_height(&mut self, drone: &mut Drone) -> i32 {
        if let Some(r) = drone.get_navdata("demo_altitude") {
            if let NavDataValue::Int(a) = r {
                return a;
            }
        }
        0
    }
}

impl Controller for ParrotController {
    fn init(&mut self) {
        let mut drone = self.drone.take().unwrap();
        match drone.startup() {
            Ok(()) => {
                thread::sleep(Duration::from_secs(2));
                drone.trim();
                thread::sleep(Duration::from_secs(2));
                drone.use_ground_cam();
                match drone.get_navdata("demo_battery") {
                    Some(NavDataValue::Uint(a)) => { println!("Battery: {}%", a); }
                    _ => { println!("Battery status unknown!"); }
                }
                thread::sleep(Duration::from_secs(2));
            }
            Err(s) => {
                panic!("Drone startup failed!");
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
        thread::sleep(Duration::from_secs(3));
        if self.print_debug {
            println!("Move UP!");
        }
        let mut current_height = self.get_current_flight_height(&mut drone);
        let mut i = 0;
        while current_height < self.flight_height {
            drone.mov_up(0.5);
            thread::sleep(Duration::from_millis(200));
            current_height = self.get_current_flight_height(&mut drone);
            if i == 50 {
                println!("{} cm was not reached within 10 seconds! (currently: {}) Stopping!", self.flight_height, current_height);
                drone.land();
                thread::sleep(Duration::from_secs(10));
                mem::drop(drone);
                panic!("Height was not reached in time!");
            }
            i += 1;
        }
        if self.print_debug {
            println!("Stop moving up!");
        }
        drone.stop();
        drone.stop();
        self.drone.replace(drone);
    }

    fn land(&mut self) {
        let mut drone = self.drone.take().unwrap();
        drone.land();
        self.drone.replace(drone);
    }

    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64) {
        if self.print_debug {
            println!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right);
            self.te.save_row("commands.txt",
                             format!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right));
        }
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
        360
    }

    fn get_video_width(&self) -> usize {
        640
    }

    fn get_opencv_url(&self) -> String {
        // TODO: Get it from the drone
        String::from("tcp://192.168.1.1:5555")
    }

    fn get_kv(&self) -> f64 {
        // TODO: NEEDS TESTING
        0.002
    }

    fn get_ka(&self) -> f64 {
        // Turning is currently turned off.
        // TODO: NEEDS TESTING
        0.0
    }
}