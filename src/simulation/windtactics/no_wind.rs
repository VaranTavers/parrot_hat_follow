use crate::simulation::traits::WindTactic;

pub struct NoWind {

}

impl NoWind {
    pub fn new() -> NoWind {
        NoWind {

        }
    }
}

impl WindTactic for NoWind {
    fn get_wind(&mut self) -> (f64, f64) {
        (0.0, 0.0)
    }
}