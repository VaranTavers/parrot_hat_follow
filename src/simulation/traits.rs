use rust_drone_follow::geometric_point::GeometricPoint;

pub trait MoveTactic {
    fn execute_move(&mut self, p: &GeometricPoint, a: f64) -> (GeometricPoint, f64);
}

pub trait WindTactic {
    fn get_wind()->(f64, f64);
}