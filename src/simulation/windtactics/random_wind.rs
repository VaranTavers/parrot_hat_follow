use crate::simulation::traits::WindTactic;
use rand::Rng;


pub struct RandomWind {
    speed_x: f64,
    speed_y: f64,
    force: f64,
    active: usize,
    inactive: usize,
    frame_num: usize,
}

impl RandomWind {
    pub fn new_polar(force: f64, active: usize, inactive: usize) -> RandomWind {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0f64, 6.283f64);
        RandomWind {
            speed_x: force * angle.sin(),
            speed_y: force * angle.cos(),
            force,
            active,
            inactive,
            frame_num: 0,
        }
    }
}

impl WindTactic for RandomWind {
    fn get_wind(&mut self) -> (f64, f64) {
        self.frame_num += 1;
        if self.frame_num < self.active {
            return (self.speed_x, self.speed_y);
        }
        if self.frame_num > self.active + self.inactive {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0f64, 6.283f64);
            self.speed_x = self.force * angle.cos();
            self.speed_y = self.force * angle.sin();
            self.frame_num = 0;
            println!("Wind on!");
        }
        (0.0, 0.0)
    }
}