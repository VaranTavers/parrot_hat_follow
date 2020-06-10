use iced::{scrollable, button, Sandbox, Element, Row, Space, Length, Column, Scrollable, Container, Button, Text};

use crate::ui::model::{TourMessage, Steps};

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
