use crate::step::Step;
use iced::{scrollable, button, Sandbox, Element, text_input, Row, Space, Length, Column, Scrollable, Container, Button, Text};
use crate::ui::step_message::StepMessage;

pub struct Tour {
    steps: Steps,
    scroll: scrollable::State,
    begin_button: button::State,
    back_button: button::State,
    next_button: button::State,
    end_button: button::State,
}

impl Sandbox for Tour {
    type Message = TourMessage;

    fn new() -> Self {
       Tour {
           steps: Steps::new(),
           scroll: scrollable::State::new(),
           begin_button: button::State::new(),
           back_button: button::State::new(),
           next_button: button::State::new(),
           end_button: button::State::new()
       }
    }

    fn title(&self) -> String {
        format!("{}", self.steps.title())
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TourMessage::StartPressed => {
                self.steps.go_to_start();
            },
            TourMessage::BackPressed => {
                self.steps.go_back();
            },
            TourMessage::NextPressed => {
                self.steps.advance();
            },
            TourMessage::EndPressed => {
                self.steps.skip_to_end();
            }
            TourMessage::StepMessage(msg) => {
                self.steps.update(msg);
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let Tour {steps,
            scroll,
            begin_button,
            back_button,
            next_button,
            end_button} = self;

        let mut controls = Row::new().spacing(10);

        if steps.has_previous() {
            controls = controls.push(Button::new(begin_button, Text::new("<<"))
                .padding(15)
                .on_press(TourMessage::StartPressed));
        }
        if steps.has_previous() {
            controls = controls.push(Button::new(back_button, Text::new("<"))
                .padding(15)
                .on_press(TourMessage::BackPressed));
        }
        controls = controls.push(Space::with_width(Length::Fill));
        if steps.can_continue() {
            controls = controls.push(Button::new(next_button, Text::new(">"))
                .padding(15)
                .on_press(TourMessage::NextPressed));
        }
        if steps.can_continue() {
            controls = controls.push(Button::new(end_button, Text::new(">>"))
                .padding(15)
                .on_press(TourMessage::EndPressed));
        }
        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(steps.view().map(TourMessage::StepMessage))
            .push(controls)
            .into();

        let scrollable = Scrollable::new(scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum TourMessage {
    StartPressed,
    BackPressed,
    NextPressed,
    EndPressed,
    StepMessage(StepMessage),
}

pub struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
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

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn skip_to_end(&mut self) {
        while self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn go_to_start(&mut self) {
        while self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&mut self) -> bool {
        self.current > 0
    }

    fn can_continue(&mut self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }

}