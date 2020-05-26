use iced::{Column, Text, Button, TextInput, Align};
use iced::text_input::State as TIS;
use iced::button::State as ButtonState;
use crate::ui::step_message::StepMessage;

pub fn set_kalman_settings<'a>(container: Column<'a, StepMessage>,
                 (s0s, sgs, vls): (&String, &String, &String),
                 (s0i, sgi, vli, si): (&'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut ButtonState)) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new().align_items(Align::Start).spacing(20)
                  .push(Text::new("Base uncertainty:"))
                  .push(TextInput::new(
                    s0i,
                    "1.0",
                    s0s.as_str(),
                    StepMessage::Sigma0).padding(15))
                  .push(Text::new("Uncertainty factor on not detected state:"))
                  .push(TextInput::new(
                    sgi,
                    "1.1",
                    sgs.as_str(),
                    StepMessage::SigmaGain).padding(15))
                  .push(Text::new("Estimated velocity loss:"))
                  .push(TextInput::new(
                    vli,
                    "0.9",
                    vls.as_str(),
                    StepMessage::VLose).padding(15)))
        .push(Button::new(si, Text::new("Save")).padding(15).on_press(StepMessage::SaveKalman))
}