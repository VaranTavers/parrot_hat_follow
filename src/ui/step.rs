use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, fs};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use iced::{button, text_input, Element, Column};

use rust_drone_follow::hat::Hat;
use rust_drone_follow::traits::Controller;
use rust_drone_follow::HatFollower;
use rust_drone_follow::detectors::naive_detector::NaiveDetector;
use rust_drone_follow::hat_file_reader::read_file;
use rust_drone_follow::hat_follower_settings::HatFollowerSettings;
use rust_drone_follow::text_exporter::TextExporter;

use super::step_message::{StepMessage, DefaultSetting};
use super::view::welcome::welcome;
use super::view::get_picture::get_picture;
use super::view::set_hat_color::set_hat_color;
use super::view::set_kalman_settings::set_kalman_settings;
use super::view::set_follower_settings::set_follower_settings;
use super::view::run::run;

use crate::utils::picture_recorder::picture_recorder;
use crate::utils::picture_funcs::{get_color_from_strings, mask_image};

use crate::parrot::parrot_controller::ParrotController;

use crate::simulation::virtual_controller::VirtualController;
use crate::simulation::movetactics::stand_still::StandStill;

use crate::kalman_filter::KalmanFilter;


pub enum Step {
    Welcome,
    GetPicture {
        drone: Option<ParrotController>,
        takeoff_state: button::State,
        picture_state: button::State,
        land_state: button::State,
        sender_channel: Option<Sender<i32>>,
        join_handle: Option<JoinHandle<()>>,
    },
    SetHatColor {
        hat: Option<Hat>,
        hls: String,
        has: String,
        hbs: String,
        lls: String,
        las: String,
        lbs: String,
        size: String,
        masked_img: String,
        save_hat: button::State,
        l_low_input: text_input::State,
        a_low_input: text_input::State,
        b_low_input: text_input::State,
        l_high_input: text_input::State,
        a_high_input: text_input::State,
        b_high_input: text_input::State,
        size_input: text_input::State,
    },
    SetKalmanSettings {
        sigma_0: String,
        sigma_gain: String,
        est_v_loss: String,
        s0_input: text_input::State,
        sg_input: text_input::State,
        vl_input: text_input::State,
        save_kalman: button::State,
    },
    SetFollowerSettings {
        min_change: String,
        center_threshold: String,
        setting: Option<DefaultSetting>,
        mc_input: text_input::State,
        ct_input: text_input::State,
        save_follower: button::State,
    },
    Run {
        sender_channel: Option<Sender<i32>>,
        join_handle: Option<JoinHandle<()>>,
        start_button: button::State,
        stop_button: button::State,
    }
}

impl<'a> Step {
    pub fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::Takeoff => {
                if let Step::GetPicture {drone, sender_channel, join_handle, ..} = self {
                    if drone.is_none() {
                        let mut controller = ParrotController::new(300, false);
                        controller.init();
                        controller.takeoff();
                        let (sx, rx) = std::sync::mpsc::channel();
                        let opencv_url = String::from("tcp://192.168.1.1:5555");
                        *join_handle = Some(thread::spawn(move || picture_recorder(rx, opencv_url)));
                        *sender_channel = Some(sx);
                        drone.replace(controller);
                    }
                }
            }
            StepMessage::TakePicture => {
                if let Step::GetPicture {sender_channel, ..} = self {
                    let channel_option = sender_channel.take();
                    if let Some(sx) = channel_option {
                        sx.send(1).unwrap();
                        sender_channel.replace(sx);
                    }
                }
            }
            StepMessage::Land => {
                if let Step::GetPicture {drone, sender_channel, join_handle, ..} = self {
                    let controller_option = drone.take();
                    if controller_option.is_some() {
                        let mut controller = controller_option.unwrap();
                        let sx = sender_channel.take().unwrap();
                        let picture_thread = join_handle.take().unwrap();
                        sx.send(0).unwrap();
                        picture_thread.join().unwrap();
                        controller.land();
                    }
                }
            }

            StepMessage::HighL(val) => {
                if let Step::SetHatColor {hls, ..} = self {
                    *hls = val;
                }
            }
            StepMessage::HighA(val) => {
                if let Step::SetHatColor {has, ..} = self {
                    *has = val;
                }
            }
            StepMessage::HighB(val) => {
                if let Step::SetHatColor {hbs, ..} = self {
                    *hbs = val;
                }
            }
            StepMessage::LowL(val) => {
                if let Step::SetHatColor {lls, ..} = self {
                    *lls = val;
                }
            }
            StepMessage::LowA(val) => {
                if let Step::SetHatColor {las, ..} = self {
                    *las = val;
                }
            }
            StepMessage::LowB(val) => {
                if let Step::SetHatColor {lbs, ..} = self {
                    *lbs = val;
                }
            }

            StepMessage::Size(val) => {
                if let Step::SetHatColor {size, ..} = self {
                    *size = val;
                }
            }

            StepMessage::SaveHat => {
                if let Step::SetHatColor {lls, las, lbs, hls, has, hbs, masked_img, size, ..} = self {
                    let low_result = get_color_from_strings(lls, las, lbs);
                    let high_result = get_color_from_strings(hls, has, hbs);

                    if let Ok(high) = high_result {
                        if let Ok(low) = low_result {
                            let system_time = SystemTime::now();
                            let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                            *masked_img = format!("image{}.png", seconds);
                            if let Ok(size_val) = mask_image("image_chosen.png", masked_img.as_str(), &low, &high) {
                                *size = format!("{}", size_val);
                                let mut text_exporter = TextExporter::new();
                                text_exporter.save_row("config.hat", String::from("./video.mp4\n"));
                                text_exporter.save_row("config.hat", format!("{} {} {}\n", lls, las, lbs));
                                text_exporter.save_row("config.hat", format!("{} {} {}\n", hls, has, hbs));
                                text_exporter.save_row("config.hat", format!("{}", size_val));
                            }
                        }
                    }
                }
            }

            StepMessage::Sigma0(val) => {
                if let Step::SetKalmanSettings {sigma_0, ..} = self {
                    *sigma_0 = val;
                }
            }

            StepMessage::SigmaGain(val) => {
                if let Step::SetKalmanSettings {sigma_gain, ..} = self {
                    *sigma_gain = val;
                }
            }

            StepMessage::VLose(val) => {
                if let Step::SetKalmanSettings {est_v_loss, ..} = self {
                    *est_v_loss = val;
                }
            }

            StepMessage::SaveKalman => {
                if let Step::SetKalmanSettings {sigma_0, sigma_gain, est_v_loss, ..} = self {
                    let mut text_exporter = TextExporter::new();
                    text_exporter.save_row("config.kalman", format!("{} {} {}", sigma_0, sigma_gain, est_v_loss));
                }
            }

            StepMessage::Center(val) => {
                if let Step::SetFollowerSettings {center_threshold, ..} = self {
                    *center_threshold = val;
                }
            }

            StepMessage::MinChange(val) => {
                if let Step::SetFollowerSettings {min_change, ..} = self {
                    *min_change = val;
                }
            }

            StepMessage::SettingChanged(val) => {
                if let Step::SetFollowerSettings {setting, ..} = self {
                    *setting = Some(val);
                }
            }

            StepMessage::SaveFollower => {
                if let Step::SetFollowerSettings {setting, center_threshold, min_change, ..} = self {
                    let mut text_exporter = TextExporter::new();
                    let setting_str = match setting {
                        Some(DefaultSetting::Video) => {
                            "Video"
                        }
                        Some(DefaultSetting::Silent) => {
                            "Silent"
                        }
                        _ => {
                            "Debug"
                        }
                    };
                    text_exporter.save_row("config.follow", format!("{} {} {}", setting_str, center_threshold, min_change));
                }
            }

            StepMessage::Start => {
                if let Step::Run {join_handle, sender_channel, ..} = self {
                    if join_handle.is_none() {

                        let follower_content = fs::read_to_string("config.follow")
                            .expect("Something went wrong reading the config.follower file");
                        let follower_args: Vec<&str> = follower_content.split(' ').collect::<Vec<&str>>();
                        let kalman_content = fs::read_to_string("config.kalman")
                            .expect("Something went wrong reading config.kalman the file");
                        let kalman_args: Vec<&str> = kalman_content.split(' ').collect::<Vec<&str>>();

                        let mut settings = match follower_args[0] {
                            "Video" => HatFollowerSettings::new(),
                            "Silent" => HatFollowerSettings::silent(),
                            _ => {
                                let mut debug = HatFollowerSettings::debug();
                                let system_time = SystemTime::now();
                                let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                                debug.save_to_file = Some(format!("video_{}.mp4", seconds));
                                debug.save_commands = Some(format!("commands_{}.txt", seconds));
                                debug
                            }
                        };
                        settings.center_threshold = match follower_args[1].trim().parse::<f64>() {
                            Ok(ct) => ct,
                            _ => 10.0
                        };
                        settings.min_change = match follower_args[2].trim().parse::<f64>() {
                            Ok(mc) => mc,
                            _ => 0.1
                        };

                        let sigma0 = match kalman_args[0].trim().parse::<f64>() {
                            Ok(mc) => mc,
                            _ => 1.0
                        };
                        let sigma_gain = match kalman_args[1].trim().parse::<f64>() {
                            Ok(mc) => mc,
                            _ => 1.1
                        };
                        let est_v_loss = match follower_args[2].trim().parse::<f64>() {
                            Ok(mc) => mc,
                            _ => 1.0
                        };


                        let (sx, rx) = std::sync::mpsc::channel();
                        let (_, hat) = read_file("config.hat");
                        // TODO: Load all files
                        *join_handle = Some(thread::spawn(move || {
                            let mut hf = HatFollower::new(
                                NaiveDetector::new(hat),
                                VirtualController::new(100.0, 0, StandStill::new(), false),
                                // ParrotController::new(300, true),
                                KalmanFilter::new(sigma0, sigma_gain, est_v_loss),
                                settings,
                                Some(rx)
                            );
                            hf.run();
                        }));
                        *sender_channel = Some(sx);
                    }
                }
            }

            StepMessage::Stop => {
                if let Step::Run {join_handle, sender_channel, ..} = self {
                    if join_handle.is_some() {
                        let follower_thread = join_handle.take().unwrap();
                        let sx = sender_channel.take().unwrap();
                        sx.send(0).unwrap();
                        follower_thread.join().unwrap();
                    }
                }
            }
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::GetPicture {..} => "Settings: Picture",
            Step::SetHatColor {..} => "Settings: Color",
            Step::SetKalmanSettings {..} => "Settings: Kalman filter",
            Step::SetFollowerSettings {..} => "Settings: HatFollower",
            Step::Run {..} => "Run",
        }
    }

    pub fn can_continue(&self) -> bool {
        match self {
            Step::Run {..} => false,
            _ => true,
        }
    }

    pub fn container() -> Column<'a, StepMessage> {
        Column::new().spacing(20)
    }

    pub fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::Welcome => welcome(Self::container()),
            Step::GetPicture { takeoff_state, picture_state, land_state, .. } => {
                get_picture(Self::container(), (takeoff_state, picture_state, land_state))
            }
            Step::SetHatColor {hls, has, hbs, lls, las, lbs, l_high_input, a_high_input, b_high_input, l_low_input, a_low_input, b_low_input, save_hat, masked_img, size, size_input, ..} => {
                set_hat_color(
                    Self::container(),
                    (hls, has, hbs, lls, las, lbs, size),
                    (l_high_input, a_high_input, b_high_input, l_low_input, a_low_input, b_low_input, size_input, save_hat),
                    masked_img
                )
            }
            Step::SetKalmanSettings {sigma_0, sigma_gain, est_v_loss, s0_input, sg_input, vl_input, save_kalman} => {
                set_kalman_settings(
                    Self::container(),
                    (sigma_0, sigma_gain, est_v_loss),
                    (s0_input, sg_input, vl_input, save_kalman)
                )
            }
            Step::SetFollowerSettings {min_change, center_threshold, mc_input, ct_input, save_follower, setting} => {
                set_follower_settings(
                    Self::container(),
                    (min_change, center_threshold),
                    (mc_input, ct_input, save_follower),
                    setting.clone()
                )
            }
            Step::Run {start_button, stop_button, ..} => {
                run(
                    Self::container(),
                    (start_button, stop_button)
                )
            }
        }.into()
    }
}

