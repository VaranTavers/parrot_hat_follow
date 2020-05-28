use crate::simulation::traits::WindTactic;

pub struct ConstantWind {
    speed_x: f64,
    speed_y: f64,
}

impl ConstantWind {
    pub fn new(speed_x: f64, speed_y: f64) -> ConstantWind {
        ConstantWind {
            speed_x,
            speed_y
        }
    }

    pub fn new_polar(force: f64, angle: f64) -> ConstantWind {
        ConstantWind {
            speed_x: force * angle.sin(),
            speed_y: force * angle.cos(),
        }
    }
}

impl WindTactic for ConstantWind {
    fn get_wind(&mut self) -> (f64, f64) {
        (self.speed_x, self.speed_y)
    }
}