use crate::simulation::traits::MoveTactic;
use rust_drone_follow::geometric_point::GeometricPoint;

pub struct StandStill {

}

impl StandStill {
    pub fn new() -> StandStill {
        StandStill {

        }
    }
}

impl MoveTactic for StandStill {
    fn execute_move(&mut self, p: &GeometricPoint, a: f64) -> (GeometricPoint, f64) {
        (p.clone(), a)
    }
}