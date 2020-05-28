use crate::simulation::traits::WindTactic;

pub struct PeriodicWind {
    speed_x: f64,
    speed_y: f64,
    active: usize,
    inactive: usize,
    frame_num: usize,
}

impl PeriodicWind {
    pub fn new(speed_x: f64, speed_y: f64, active: usize, inactive: usize) -> PeriodicWind {
        PeriodicWind {
            speed_x,
            speed_y,
            active,
            inactive,
            frame_num: 0,
        }
    }

    pub fn new_polar(force: f64, angle: f64, active: usize, inactive: usize) -> PeriodicWind {
        PeriodicWind {
            speed_x: force * angle.sin(),
            speed_y: force * angle.cos(),
            active,
            inactive,
            frame_num: 0,
        }
    }
}

impl WindTactic for PeriodicWind {
    fn get_wind(&mut self) -> (f64, f64) {
        if self.frame_num < self.active {
            return (self.speed_x, self.speed_y);
        }
        self.frame_num += 1;
        if self.frame_num > self.active + self.inactive {
            self.frame_num = 0;
        }
        (0.0, 0.0)
    }
}