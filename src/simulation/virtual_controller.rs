use std::thread;
use std::time::Duration;
use std::mem;

use rust_drone_follow::traits::Controller;
use rust_drone_follow::text_exporter::TextExporter;
use rust_drone_follow::opencv_custom::{get_red, get_green};
use rust_drone_follow::point_converter::PointConverter;
use rust_drone_follow::geometric_point::GeometricPoint;

use opencv::imgproc::{circle, LINE_8};
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::core::{Mat, Size, CV_8UC3, MatExprTrait, MatTrait, Point, Scalar};

use crate::simulation::traits::MoveTactic;
use crate::simulation::traits::WindTactic;


pub struct VirtualController<M: MoveTactic> {
    print_debug: bool,
    default_image: Mat,
    te: TextExporter,
    p_c: PointConverter,
    drone: GeometricPoint,
    drone_v: (f64, f64),
    hat: (GeometricPoint, f64),
    tactic: M,
    speed: f64,
    skip_frames: u32,
}

impl<M: MoveTactic> VirtualController<M> {
    pub fn new(speed: f64, skip_frames: u32, tactic: M, debug: bool) -> VirtualController<M> {
        VirtualController {
            print_debug: debug,
            default_image: Mat::ones(320, 640, CV_8UC3).unwrap().to_mat().unwrap(),
            p_c: PointConverter::new(640, 320),
            te: TextExporter::new(),
            drone: GeometricPoint::new(0, 0),
            drone_v: (0.0, 0.0),
            hat: (GeometricPoint::new(30, 45), 0.0),
            speed,
            skip_frames,
            tactic,
        }
    }

    pub fn draw_hat(&self, img: &mut Mat, hat_pos: &GeometricPoint, drone_pos: &GeometricPoint, angle: f64) {
       let new_point = GeometricPoint::new(hat_pos.x - drone_pos.x, hat_pos.y - drone_pos.y);

       circle(img, self.p_c.convert_to_image_coords(&new_point), 25, get_red(), -1, LINE_8, 0);
       circle(img, self.p_c.convert_to_image_coords(&new_point), 20, Scalar::new(76.0, 76.0, 205.0, 255.0), -1, LINE_8, 0);
    }

    pub fn draw_background(&self, img: &mut Mat, drone_pos: &GeometricPoint) {
        for i in 1..5 {
            for j in 1..10 {
                let tree_x = 20 + (1000 / 10 * j) - 500;
                let tree_y = 10 + (500 / 5 * i) - 250;
                let new_point = GeometricPoint::new(tree_x - drone_pos.x, tree_y - drone_pos.y);
                circle(img, self.p_c.convert_to_image_coords(&new_point), 10, get_green(), -1, LINE_8, 0);
            }
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
        self.drone = GeometricPoint::new(new_x as i32, new_y as i32);

        let (&old_hat, old_angle) = self.hat;
        let (hat, angle) = self.tactic.execute_move(old_hat, old_angle);
        self.hat = (hat, angle);

        *img = Mat::ones(320, 640, CV_8UC3).unwrap().to_mat().unwrap();

        self.draw_background(img, &self.drone);
        // Move happened, error will probably occur
        self.draw_hat(img, &hat, &self.drone, angle);

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