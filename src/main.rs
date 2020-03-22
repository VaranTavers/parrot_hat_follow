use std::{thread, io};
use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::naive_detector::NaiveDetector;
use rust_drone_follow::hat_file_reader::read_file;
use rust_drone_follow::filters::no_filter::NoFilter;

mod parrot_controller;

use parrot_controller::ParrotController;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
fn read_int() -> i32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    parse_input!(input_line.trim(), i32)
}

fn main() {
    let (mut sx, rx) = std::sync::mpsc::channel();
    let (_, hat) = read_file("kek.hat");

    let handle = thread::spawn(|| {
        let mut hf = HatFollower::new(
            NaiveDetector::new(hat),
            ParrotController::new(),
            NoFilter::new(),
            Some(rx)
        );

        hf.run();
    });

    loop {
        let i = read_int();
        if i == 0 {
            sx.send(0);
            break;
        }
    }

    handle.join().unwrap();
}
