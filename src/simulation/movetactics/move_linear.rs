use crate::simulation::traits::MoveTactic;

pub struct MoveLinear {
    speed_x: f64,
    speed_y: f64,
}

impl MoveLinear {
    pub fn new(speed_x: f64, speed_y: f64) -> MoveLinear {
        MoveLinear {
            speed_x,
            speed_y,
        }
    }
}

impl MoveTactic for MoveLinear {
    fn execute_move(&mut self, x: f64, y: f64, a: f64) -> (f64, f64, f64) {
        (x + self.speed_x, y + self.speed_y, a)
    }
}