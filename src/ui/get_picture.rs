use iced::{Column, Text, Button, Row, Align};
use iced::button::State as ButtonState;
use crate::ui::step_message::StepMessage;

pub fn get_picture<'a>(container: Column<'a, StepMessage>, (ts, ps, ls): (&'a mut ButtonState, &'a mut ButtonState, &'a mut ButtonState)) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new()
            .align_items(Align::Start)
            .spacing(10)
            .push(Text::new("If you want to set up the drone to follow a new hat, you can take \
            a picture of it, and configure the colors on the next tab."))
            .push(Text::new("Please make sure you are connected to the drone, then push the Takeoff button!"))
            .push(Text::new("A new window will appear with the drone's camera picture. Push the Take Picture button when the hat is visible!"))
            .push(Text::new("After you are done, stay clear of the landing zone, and push the Land button!")))
        .push(Row::new()
            .spacing(10)
            .push(Button::new(ts, Text::new("Takeoff")).padding(15).on_press(StepMessage::Takeoff))
            .push(Button::new(ps, Text::new("Take Picture")).padding(15).on_press(StepMessage::TakePicture))
            .push(Button::new(ls, Text::new("Land")).padding(15).on_press(StepMessage::Land)))
}