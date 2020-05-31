
use rulinalg::vector::Vector;
use linearkalman::{KalmanFilter as KF, filter_step};
use linearkalman::KalmanState as KS;
use rust_drone_follow::traits::Filter;
use rust_drone_follow::geometric_point::GeometricPoint;
use rust_drone_follow::marker_drawer::MarkerDrawer;
use rust_drone_follow::opencv_custom::get_blue;
use rulinalg::matrix::{Matrix};
use opencv::core::Scalar;

pub struct KalmanFilter {
    filter: KF,
    state: KS,
    point: Option<GeometricPoint>,
    angle: f64,
    sigma_gain: f64,
}

impl KalmanFilter {
    /// Creates a new KalmanFilter with the given properties.
    /// Parameters:
    /// sigma0: is the uncertainty of the initial estimation (p0 will be an eye(sigma0))
    /// sigma_gain: is the factor with which uncertainty grows in case the hat is not detected on the screen (1.0 means no change)
    /// est_v_loss: is the factor with which the Kalman Filter estimates loss in vx, vy (due to the drone adapting to the moving hat) (1.0 means no change)
    pub fn new(sigma0: f64, sigma_gain: f64, est_v_loss: f64) -> KalmanFilter {
        KalmanFilter {
            filter : KF {
                // Process noise covariance
                q: Matrix::new(5, 5, vec![ 1.0, 0.0, 0.0, 0.0, 0.0,
                                           0.0, 1.0, 0.0, 0.0, 0.0,
                                           0.0, 0.0, 2.0, 0.0, 0.0,
                                           0.0, 0.0, 0.0, 1.0, 0.0,
                                           0.0, 0.0, 0.0, 0.0, 1.0 ]),
                // Measurement noise matrix
                r: Matrix::new(3, 3, vec![ 10.0, 0.0, 0.0,
                                           0.0, 10.0, 0.0,
                                           0.0, 0.0, 0.001 ]),
                // Observation matrix
                h: Matrix::new(3, 5, vec![1.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 1.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 1.0, 0.0, 0.0]),
                // State transition matrix
                f: Matrix::new(5, 5, vec![ 1.0, 0.0, 0.0, 1.0, 0.0,
                            0.0, 1.0, 0.0, 0.0, 1.0,
                            0.0, 0.0, 1.0, 0.0, 0.0,
                            0.0, 0.0, 0.0, est_v_loss, 0.0,
                            0.0, 0.0, 0.0, 0.0, est_v_loss ]),
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
                x: Vector::new(vec![ 0.0, 0.0, 0.0, 0.0, 0.0 ]),
                // Initial guess for state covariance at time 1
                p: Matrix::new(5, 5, vec![sigma0, 0.0, 0.0, 0.0, 0.0,
                                          0.0, sigma0, 0.0, 0.0, 0.0,
                                          0.0, 0.0, sigma0, 0.0, 0.0,
                                          0.0, 0.0, 0.0, sigma0, 0.0,
                                          0.0, 0.0, 0.0, 0.0, sigma0]),
            },
            point: None,
            angle: 0.0,
            sigma_gain,
        }
    }
}

impl Filter for KalmanFilter {
    fn update_estimation(&mut self, point: Option<GeometricPoint>, angle: Option<f64>, _cert: f64) {
        match point {
            Some(p) => {
                if let Some(a) = angle {
                    let (next, _pred) = filter_step(&self.filter,
                                                    &self.state,
                                                    &vector![p.x as f64, p.y as f64, a],
                    );

                    self.point = Some(GeometricPoint::new(next.x[0] as i32, next.x[1] as i32));
                    self.angle = next.x[2];
                    self.state = next;
                }
            }
            None => {
                self.state.p = &self.state.p * self.sigma_gain;
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

            let k = 10;
            let other_point = GeometricPoint::new(p.x + k, p.y + (k as f64 * self.angle.max(-1.57).min(1.57).tan()) as i32);

            m_d.line(p, &other_point, Scalar::new(255.0, 255.0, 255.0, 255.0));
        }
    }
}
