use iced::{button, text_input, Element};

use crate::ui::step::Step;
use crate::ui::model::StepMessage;

pub struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    pub fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::SetController {
                    cs: None,
                    ws: None,
                    ps: None,
                    save_controller: button::State::new(),
                },
                Step::GetPicture {
                    drone: None,
                    takeoff_state: button::State::new(),
                    picture_state: button::State::new(),
                    land_state: button::State::new(),
                    sender_channel: None,
                    join_handle: None
                },
                Step::SetHatColor {
                    hat: None,
                    hls: "".to_string(),
                    has: "".to_string(),
                    hbs: "".to_string(),
                    lls: "".to_string(),
                    las: "".to_string(),
                    lbs: "".to_string(),
                    size: "".to_string(),
                    masked_img: "".to_string(),
                    save_hat: button::State::new(),
                    l_low_input: text_input::State::new(),
                    a_low_input: text_input::State::new(),
                    b_low_input: text_input::State::new(),
                    l_high_input: text_input::State::new(),
                    a_high_input: text_input::State::new(),
                    b_high_input: text_input::State::new(),
                    size_input: text_input::State::new()
                },
                Step::SetKalmanSettings {
                    sigma_0: "".to_string(),
                    sigma_gain: "".to_string(),
                    est_v_loss: "".to_string(),
                    s0_input: text_input::State::new(),
                    sg_input: text_input::State::new(),
                    vl_input: text_input::State::new(),
                    save_kalman: button::State::new()
                },
                Step::SetFollowerSettings {
                    min_change: "".to_string(),
                    center_threshold: "".to_string(),
                    setting: None,
                    mc_input: text_input::State::new(),
                    ct_input: text_input::State::new(),
                    save_follower: button::State::new()
                },
                Step::Run {
                    sender_channel: None,
                    join_handle: None,
                    start_button: button::State::new(),
                    stop_button: button::State::new()
                }
            ],
            current: 0,
        }
    }

    pub fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    pub fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    pub fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    pub fn skip_to_end(&mut self) {
        while self.can_continue() {
            self.current += 1;
        }
    }

    pub fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    pub fn go_to_start(&mut self) {
        while self.has_previous() {
            self.current -= 1;
        }
    }

    pub fn has_previous(&mut self) -> bool {
        self.current > 0
    }

    pub fn can_continue(&mut self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    pub fn title(&self) -> &str {
        self.steps[self.current].title()
    }

}
