use std::sync::mpsc::Receiver;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::core::{Mat, Size, CV_8U, MatExprTrait};
use opencv::imgcodecs::imwrite;
use opencv::types::VectorOfi32;
use opencv::highgui::imshow;

pub fn picture_recorder(rec: Receiver<i32>, url: String) {
    let mut video = VideoCapture::from_file(url.as_str(), CAP_ANY).unwrap();
    let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();

    loop {
        if let Ok(a) = rec.try_recv() {
            if a == 1 {
                imwrite("image_chosen.png", &img, &VectorOfi32::new()).unwrap();
            } else {
                opencv::highgui::destroy_all_windows().unwrap();
                break;
            }
        }
        match video.read(&mut img) {
            Ok(true) => {
                imshow("Camera Picture",&img).unwrap();
                opencv::highgui::wait_key(3).unwrap();
            }
            _ => {
                break;
            }
        }
    }
}