use rust_drone_follow::traits::Controller;

/// The MockController acts as a false controller that provides a video file to the MainFrame along
/// with it's resolution, and does nothing on commands given to it.
///
/// You can use it to test the tracking system on a prerecorded video.
pub struct MockPrinterController {
    filename: String,
    height: usize,
    width: usize,
}

impl MockPrinterController {
    pub fn new(filename: &str, width: usize, height: usize) -> MockPrinterController {
        MockPrinterController {
            filename: String::from(filename),
            height,
            width
        }
    }
}

impl Controller for MockPrinterController {
    fn init(&mut self) {
        println!("Initializing the drone.");
    }
    fn shutdown(&mut self) {
        println!("Shutting down the drone.");
    }

    fn takeoff(&mut self) {
        println!("Takeoff");
    }
    fn land(&mut self) {
        println!("Land");
    }

    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64) {
        println!("Movin' Cruisin: {}, {}, {}, {}", left_right, back_front, down_up, turn_left_right);
    }

    /// Should halt all movement
    fn stop(&mut self) {
        println!("Halt all movement.");
    }

    fn get_video_height(&self) -> usize {
        self.height
    }

    fn get_video_width(&self) -> usize {
        self.width
    }

    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String {
        self.filename.clone()
    }

    /// Conversion rate between pixels/dt and drone speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames
    fn get_kv(&self) -> f64 {
        0.01
    }

    /// Conversion rate between da/dt and drone turn speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames, and da is the angle difference between frames.
    fn get_ka(&self) -> f64 {
        0.01
    }
}
