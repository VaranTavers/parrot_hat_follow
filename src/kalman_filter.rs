
use rulinalg::vector::Vector;
use linearkalman::{KalmanFilter as KF, filter_step};
use linearkalman::KalmanState as KS;
use rust_drone_follow::traits::Filter;
use rust_drone_follow::geometric_point::GeometricPoint;
use rust_drone_follow::marker_drawer::MarkerDrawer;
use rust_drone_follow::opencv_custom::get_blue;
use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};
use std::ops::Index;

pub struct KalmanFilter {
    filter: KF,
    state: KS,
    point: Option<GeometricPoint>,
    angle: f64,
}

impl KalmanFilter {
    pub fn new() -> KalmanFilter {
        KalmanFilter {
            filter : KF {
                // Process noise covariance
                q: Matrix::new(5, 5, vec![ 1.0, 0.0, 0.0, 0.0, 0.0,
                                           0.0, 1.0, 0.0, 0.0, 0.0,
                                           0.0, 0.0, 1.0, 0.0, 0.0,
                                           0.0, 0.0, 0.0, 1.0, 0.0,
                                           0.0, 0.0, 0.0, 0.0, 1.0 ]),
                // Measurement noise matrix
                r: Matrix::new(3, 3, vec![ 1.0, 0.0, 0.0,
                                           0.0, 1.0, 0.0,
                                           0.0, 0.0, 1.0 ]),
                // Observation matrix
                h: Matrix::new(3, 5, vec![1.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 1.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 1.0, 0.0, 0.0]),
                // State transition matrix
                f: Matrix::new(5, 5, vec![ 1.0, 0.0, 0.0, 1.0, 0.0,
                            0.0, 1.0, 0.0, 0.0, 1.0,
                            0.0, 0.0, 1.0, 0.0, 0.0,
                            0.0, 0.0, 0.0, 1.0, 0.0,
                            0.0, 0.0, 0.0, 0.0, 1.0 ]),
                // Initial guess for state mean at time 1
                x0: Vector::new(vec![ 0.0, 0.0, 0.0, 0.0, 0.0 ]),
                // Initial guess for state covariance at time 1
                p0: Matrix::new(5, 5, vec![ 1.0, 0.0, 0.0, 0.0, 0.0,
                             0.0, 1.0, 0.0, 0.0, 0.0,
                             0.0, 0.0, 1.0, 0.0, 0.0,
                             0.0, 0.0, 0.0, 1.0, 0.0,
                             0.0, 0.0, 0.0, 0.0, 1.0 ]),
            },
            state: KS {
                // Initial guess for state mean at time 1
                x: Vector::new(vec![ 10.0, 10.0, 0.0, 1.0, 0.0 ]),
                // Initial guess for state covariance at time 1
                p: Matrix::new(5, 5, vec![ 10.0, 0.0, 0.0, 0.0, 0.0,
                             0.0, 10.0, 0.0, 0.0, 0.0,
                             0.0, 0.0, 10.0, 0.0, 0.0,
                             0.0, 0.0, 0.0, 10.0, 0.0,
                             0.0, 0.0, 0.0, 0.0, 10.0 ]),
            },
            point: None,
            angle: 0.0,
        }
    }
}

impl Filter for KalmanFilter {
    fn update_estimation(&mut self, point: Option<GeometricPoint>, angle: Option<f64>, cert: f64) {
        match point {
            Some(p) => {
                if let Some(a) = angle {
                    let (next, _pred) = filter_step(&self.filter,
                                                    &self.state,
                                                    &vector![p.x as f64, p.y as f64, a],
                    );

                    self.point = Some(GeometricPoint::new(next.x[0] as i32, next.x[1] as i32));
                    self.angle = next.x[3];
                    self.state = next;
                }
            }
            None => {
                let uncertainty_increase = 1.1;
                self.state.p = &self.state.p * uncertainty_increase;
            }
        }
    }

    fn get_estimated_position(&self) -> Option<GeometricPoint> {
        self.point.as_ref().map(|p| p.clone())
    }

    fn get_estimated_angle(&self) -> f64 {
        self.angle
    }

    fn get_estimated_vx(&self) -> f64 {
        self.state.x[3]
    }

    fn get_estimated_vy(&self) -> f64 {
        self.state.x[4]
    }

    fn get_estimation_certainty(&self) -> f64 {
        1.0
    }

    fn draw_on_image(&self, m_d: &mut MarkerDrawer) {
        if let Some(p) = &self.point {
            m_d.point(p, get_blue());
        }
    }
}
