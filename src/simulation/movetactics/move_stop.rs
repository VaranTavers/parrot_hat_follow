use crate::simulation::traits::MoveTactic;

use rust_drone_follow::models::GeometricPoint;

pub struct MoveStop {
    speed: f64,
    switch_time: u32,
    frame: u32,
}

impl MoveStop {
    pub fn new(speed: f64, switch_time: u32) -> MoveStop {
        MoveStop {
            speed,
            switch_time,
            frame: 0,
        }
    }
}

impl MoveTactic for MoveStop {
    fn execute_move(&mut self, x: f64, y: f64, a: f64) -> (f64, f64, f64) {
        self.frame += 1;
        if self.frame < self.switch_time {
            return (x + self.speed * a.cos(), y + self.speed * a.sin(), a);
        }
        if self.frame == 2 * self.switch_time {
            self.frame = 0;
        }
        (x, y, a)
    }
}