pub trait MoveTactic {
    fn execute_move(&mut self, x: f64, y: f64, a: f64) -> (f64, f64, f64);
}

pub trait WindTactic {
    fn get_wind(&mut self)->(f64, f64);
}