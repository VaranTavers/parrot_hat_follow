use crate::move_tactic_trait::MoveTactic;

pub struct StandStill {

}

impl MoveTactic for StandStill {
    fn execute_move(&mut self, x: f64, y: f64, a: f64) -> (f64, f64, f64) {
        (x, y, a)
    }
}