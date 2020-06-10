use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use iced::{button, text_input, Element, Column};

use rust_drone_follow::traits::Controller;
use rust_drone_follow::models::Hat;
use rust_drone_follow::utils::TextExporter;

use super::view::welcome;
use super::view::set_controller_settings;
use super::view::get_picture;
use super::view::set_hat_color;
use super::view::set_kalman_settings;
use super::view::set_follower_settings;
use super::view::run;

use crate::ui::model::{StepMessage, DefaultSetting, ControllerSetting, WindSetting, PersonSetting};

use crate::utils::picture_recorder::picture_recorder;
use crate::utils::picture_funcs::{get_color_from_strings, mask_image};

use crate::parrot::parrot_controller::ParrotController;

pub enum Step {
    Welcome,
    SetController {
        cs: Option<ControllerSetting>,
        ws: Option<WindSetting>,
        ps: Option<PersonSetting>,
        save_controller: button::State,
    },
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
            StepMessage::SetController(controller) => {
                if let Step::SetController {cs, ..} = self {
                    *cs = Some(controller);
                }
            },
            StepMessage::SetWind(wind) => {
                if let Step::SetController {ws, ..} = self {
                    *ws = Some(wind);
                }
            },
            StepMessage::SetPerson(person) => {
                if let Step::SetController {ps, ..} = self {
                    *ps = Some(person);
                }
            },
            StepMessage::SaveController => {
                if let Step::SetController {cs, ws, ps, ..} = self {
                    let mut text_exporter = TextExporter::new();
                    text_exporter.save_row("config.controller", format!("{}\n", String::from(cs.unwrap())));
                    text_exporter.save_row("config.controller", format!("{}\n", String::from(ws.unwrap())));
                    text_exporter.save_row("config.controller", format!("{}", String::from(ps.unwrap())));
                }
            }
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
                        Some(DefaultSetting::Video) => "Video",
                        Some(DefaultSetting::Silent) => "Silent",
                        _ => "Debug"
                    };
                    text_exporter.save_row("config.follow", format!("{} {} {}", setting_str, center_threshold, min_change));
                }
            }

            StepMessage::Start => {
                if let Step::Run {join_handle, sender_channel, ..} = self {
                    if join_handle.is_none() {
                        let (handle, sx) = crate::ui::controller::start_follow();
                        *join_handle = Some(handle);
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
            Step::SetController {..} => "Settings: Controller",
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
            Step::SetController { cs, ws, ps, save_controller } => {
                set_controller_settings(Self::container(), save_controller, (cs.clone(), ws.clone(), ps.clone()))
            },
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

