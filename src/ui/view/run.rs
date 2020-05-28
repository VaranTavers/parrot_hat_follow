use iced::{Column, Text, Button, Align};
use iced::button::State as ButtonState;

use crate::ui::step_message::StepMessage;

pub fn run<'a>(container: Column<'a, StepMessage>, (start_state, stop_state): (&'a mut ButtonState, &'a mut ButtonState)) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Great! The drone will follow you from now on. You can start it up!"))
            .push(Text::new("Please make sure you are connected to the drone, then push the Start button!"))
            .push(Text::new("If Video or Debug mode is set, a new window will appear with the drone's camera picture."))
            .push(Text::new("After you are done, stay clear of the landing zone, and push the Stop button!")))
        .push(Button::new(start_state, Text::new("Start")).padding(15).on_press(StepMessage::Start))
        .push(Button::new(stop_state, Text::new("Stop")).padding(15).on_press(StepMessage::Stop))
}