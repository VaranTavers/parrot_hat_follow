use std::f64::consts::PI;

use crate::simulation::traits::MoveTactic;

use rust_drone_follow::models::GeometricPoint;

pub struct StandTurn {
    turn_speed: f64,
}

impl StandTurn {
    pub fn new(turn_speed: f64) -> StandTurn {
        StandTurn {
            turn_speed,
        }
    }
}

impl MoveTactic for StandTurn {
    fn execute_move(&mut self, x: f64, y: f64, a: f64) -> (f64, f64, f64) {
        (x, y, a - self.turn_speed)
    }
}