use std::f64::consts::PI;

use rust_drone_follow::traits::Controller;

use rust_drone_follow::utils::TextExporter;
use rust_drone_follow::utils::opencv_custom::{get_red, get_green};
use rust_drone_follow::utils::PointConverter;

use rust_drone_follow::models::GeometricPoint;

use opencv::imgproc::{circle, LINE_8};
use opencv::core::{Mat, CV_8UC3, MatExprTrait, Scalar};

use crate::simulation::traits::MoveTactic;
use crate::simulation::traits::WindTactic;

use rand::Rng;


pub struct VirtualController<M: MoveTactic, W: WindTactic> {
    print_debug: bool,
    te: TextExporter,
    p_c: PointConverter,
    drone: (f64, f64, f64),
    drone_v: (f64, f64, f64),
    hat: (f64, f64, f64),
    move_tactic: M,
    wind_tactic: W,
    speed: f64,
    skip_frames: u32,
    instability: f64,
}

impl<M: MoveTactic, W: WindTactic> VirtualController<M, W> {
    pub fn new(speed: f64, skip_frames: u32, instability: f64, move_tactic: M, wind_tactic: W, debug: bool) -> VirtualController<M, W> {
        VirtualController {
            print_debug: debug,
            p_c: PointConverter::new(640, 320),
            te: TextExporter::new(),
            drone: (0.0, 0.0, 1.57),
            drone_v: (0.0, 0.0, 0.0),
            hat: (30.0, 45.0, 0.0), //1.57
            speed,
            skip_frames,
            instability,
            move_tactic,
            wind_tactic,
        }
    }

    pub fn turn_by(&self, (x, y): (f64, f64), a: f64) -> (f64, f64) {
        let pipk = PI / 2.0;
        (
            (pipk - a).cos() * (x as f64) - (pipk - a).sin() * (y as f64),
            (pipk - a).sin() * (x as f64) + (pipk - a).cos() * (y as f64),
        )
    }

    pub fn draw_hat(&self, img: &mut Mat, (hat_x, hat_y, hat_ang): (f64, f64, f64), (drone_x, drone_y, drone_ang): (f64, f64, f64)) {
        let (turned_x, turned_y) = self.turn_by((hat_x, hat_y), drone_ang);
        let new_point = GeometricPoint::new((turned_x - drone_x) as i32, (turned_y - drone_y) as i32);
        let new_angle = hat_ang + 1.57 - drone_ang;

        let front_point = GeometricPoint::new(
            new_point.x + (new_angle.cos() * 22.0) as i32,
            new_point.y + (new_angle.sin() * 22.0) as i32
        );

        // Base
        circle(img, self.p_c.convert_to_image_coords(&new_point), 25, get_red(), -1, LINE_8, 0).unwrap();
        // Front
        circle(img, self.p_c.convert_to_image_coords(&front_point), 25, get_red(), -1, LINE_8, 0).unwrap();
        // Other color on base
        circle(img, self.p_c.convert_to_image_coords(&new_point), 20, Scalar::new(76.0, 76.0, 205.0, 255.0), -1, LINE_8, 0).unwrap();
    }

    pub fn draw_background(&self, img: &mut Mat, (drone_x, drone_y, drone_ang): (f64, f64, f64)) {
        for i in 1..5 {
            for j in 1..10 {
                let tree_x = 20 + (1000 / 10 * j) - 500;
                let tree_y = 10 + (500 / 5 * i) - 250;
                let (turned_x, turned_y) = self.turn_by((tree_x as f64, tree_y as f64), drone_ang);
                let new_point = GeometricPoint::new((turned_x - drone_x) as i32, (turned_y - drone_y) as i32);
                circle(img, self.p_c.convert_to_image_coords(&new_point), 10, get_green(), -1, LINE_8, 0).unwrap();
            }
        }
    }
}

impl<M: MoveTactic, W: WindTactic> Controller for VirtualController<M, W> {
    fn init(&mut self) { }

    fn shutdown(&mut self) { }

    fn takeoff(&mut self) { }

    fn land(&mut self) { }

    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64) {
        // TODO: Move shouldn't be instant
        if self.print_debug {
            println!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right);
            self.te.save_row("commands.txt",
                             format!("{}, {}, {}, {}", left_right, back_front, down_up, turn_left_right));
        }
        let (old_vx, old_vy, old_va) = self.drone_v;
        self.drone_v = ((old_vx + left_right) / 2.0, (old_vy + back_front) / 2.0, (old_va + turn_left_right) / 2.0);
    }

    fn stop(&mut self) {
        self.drone_v = (0.0, 0.0, 0.0);
    }

    fn get_video_height(&self) -> usize {
        360
    }

    fn get_video_width(&self) -> usize {
        640
    }

    fn get_next_frame(&mut self, img: &mut Mat) -> opencv::Result<bool> {
        let mut rng = rand::thread_rng();

        for _i in 0..(1 + self.skip_frames) {
            let (last_x, last_y, last_a) = self.drone;
            let (v_x, v_y, v_a) = self.drone_v;
            let (wind_x, wind_y) = self.wind_tactic.get_wind();
            let (inst_x, inst_y) = (
                rng.gen_range(- self.instability, self.instability),
                rng.gen_range(- self.instability, self.instability)
            );
            let (new_x, new_y, new_a) = (
                last_x as f64 + self.speed * v_x + wind_x + inst_x,
                last_y as f64 + self.speed * v_y + wind_y + inst_y,
                last_a + v_a
            );
            self.drone = (new_x, new_y, new_a);

            let (old_hat_x, old_hat_y, old_angle) = self.hat;
            self.hat = self.move_tactic.execute_move(old_hat_x, old_hat_y, old_angle);

        }

        *img = Mat::ones(self.get_video_height() as i32, self.get_video_width() as i32, CV_8UC3).unwrap().to_mat().unwrap();

        self.draw_background(img, self.drone);
        self.draw_hat(img, self.hat, self.drone);

        Ok(true)
    }

    fn get_kv(&self) -> f64 {
        // TODO: NEEDS TESTING
        0.003
    }

    fn get_ka(&self) -> f64 {
        // Turning is currently turned off.
        // TODO: NEEDS TESTING
        0.01
    }
}