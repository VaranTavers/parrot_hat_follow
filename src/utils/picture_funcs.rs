use opencv::imgproc::{COLOR_BGR2Lab, cvt_color, contour_area};
use opencv::core::*;
use opencv::imgcodecs::{imread, IMREAD_COLOR, imwrite};
use opencv::types::{VectorOfi32};
use opencv::prelude::Vector;

use rust_drone_follow::opencv_custom::{LabColor, mat_size_of_other, get_mask, get_contours};

pub fn get_masked(a: &Mat, lower_bound: &LabColor, upper_bound: &LabColor) -> Mat {
    let mut hsv = mat_size_of_other(a);
    cvt_color(a, &mut hsv, COLOR_BGR2Lab, 0).unwrap();

    let mask = get_mask(&hsv, lower_bound, upper_bound);
    let mut output = mat_size_of_other(&hsv);
    let mut thresh: Mat = mat_size_of_other(&hsv);

    opencv::core::bitwise_and(&a, &a, &mut output, &mask).unwrap();
    opencv::imgproc::threshold(&mask, &mut thresh, 40.0, 255.0, 0).unwrap();

    output
}

pub fn mask_image(input: &str, output: &str, low: &LabColor, high: &LabColor) -> opencv::Result<f64> {
    let original = imread(input, IMREAD_COLOR)?;
    let masked = get_masked(&original, low, high);
    imwrite(output, &masked, &VectorOfi32::new())?;

    let contours = get_contours(&original, low, high);
    let c_area = contours.iter()
        .map(|contour| contour_area(&contour, false).unwrap())
        .collect::<Vec<f64>>();

    Ok(c_area.iter().fold(0.0, |init, value| {
        if *value > init {
            return *value;
        }
        init
    }))
}

pub fn get_color_from_strings(l: &String, a: &String, b: &String) -> Result<LabColor, std::num::ParseIntError> {
    let ll = l.trim().parse::<i8>()?;
    let aa = a.trim().parse::<i8>()?;
    let bb = b.trim().parse::<i8>()?;

    Ok(LabColor::new(ll, aa, bb))
}