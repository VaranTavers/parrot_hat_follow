use std::thread;
use std::time::Duration;
use std::mem;

use rust_drone_follow::traits::Controller;
use rust_drone_follow::text_exporter::TextExporter;
use parrot_ar_drone::{Drone, NavDataValue};
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::core::{Mat, Size, CV_8UC3, MatExprTrait, MatTrait, Point, Scalar};
use crate::move_tactic_trait::MoveTactic;
use opencv::imgproc::{circle, LINE_8};
use rust_drone_follow::opencv_custom::get_red;

pub struct VirtualController<M: MoveTactic> {
    print_debug: bool,
    default_image: Mat,
    te: TextExporter,
    drone: (f64, f64),
    drone_v: (f64, f64),
    hat: (f64, f64, f64),
    tactic: M,
    speed: f64,
}

impl<M: MoveTactic> VirtualController<M> {
    pub fn new(speed: f64, tactic: M, debug: bool) -> VirtualController<M> {
        VirtualController {
            print_debug: debug,
            default_image: Mat::ones(640, 320, CV_8UC3).unwrap().to_mat().unwrap(),
            te: TextExporter::new(),
            drone: (0.0, 0.0),
            drone_v: (0.0, 0.0),
            hat: (0.0, 35.0, 0.0),
            speed,
            tactic,
        }
    }
}

impl<M: MoveTactic> Controller for VirtualController<M> {
    fn init(&mut self) { }

    fn shutdown(&mut self) { }

    fn takeoff(&mut self) { }

    fn land(&mut self) { }

    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64) {
        if self.print_debug {
            println!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right);
            self.te.save_row("commands.txt",
                             format!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right));
        }
        self.drone_v = (left_right, back_front);
    }

    fn stop(&mut self) {
        self.drone_v = (0.0, 0.0);
    }

    fn get_video_height(&self) -> usize {
        360
    }

    fn get_video_width(&self) -> usize {
        640
    }

    fn get_next_frame(&mut self, img: &mut Mat) -> opencv::Result<bool> {
        let (last_x, last_y) = self.drone;
        let (v_x, v_y) = self.drone_v;
        let (new_x, new_y) = (last_x + self.speed * v_x, last_y + self.speed * v_y);
        self.drone = (new_x, new_y);

        let (old_hat_x, old_hat_y, old_angle) = self.hat;
        let (hat_x, hat_y, angle) = self.tactic.execute_move(old_hat_x, old_hat_y, old_angle);
        self.hat = (hat_x, hat_y, angle);


        *img = opencv::core::Mat::copy(& self.default_image).unwrap();
        let (draw_x, draw_y) = (
            (hat_x - new_x + (self.get_video_width() / 2) as f64) as i32,
            (self.get_video_height() as f64 / 2.0 - (hat_y - new_y)) as i32);

        circle(img, Point::new(draw_x, draw_y), 30, get_red(), -1, LINE_8, 0);
        circle(img, Point::new(draw_x, draw_y), 10, Scalar::new(76.0, 76.0, 255.0, 255.0), -1, LINE_8, 0);
        Ok(true)
    }

    fn get_kv(&self) -> f64 {
        // TODO: NEEDS TESTING
        0.003
    }

    fn get_ka(&self) -> f64 {
        // Turning is currently turned off.
        // TODO: NEEDS TESTING
        0.0
    }
}