use iced::{Column, Text, Button, Row, Align, TextInput, Length, Image};
use iced::text_input::State as TIS;
use iced::button::State as ButtonState;

use crate::ui::step_message::StepMessage;

pub fn set_hat_color<'a>(container: Column<'a, StepMessage>,
                         (hls, has, hbs, lls, las, lbs, size): (&String, &String, &String, &String, &String, &String, &String),
                         (hli, hai, hbi, lli, lai, lbi, sizei, si): (&'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut TIS, &'a mut ButtonState),
                         masked_img: &String) -> Column<'a, StepMessage> {

    let high_row = Row::new().spacing(5)
        .padding(10)
        .align_items(Align::Center)
        .push(Text::new("L: "))
        .push(TextInput::new(
            hli,
            "0",
            hls.as_str(),
            StepMessage::HighL).padding(15))
        .push(Text::new("a: "))
        .push(TextInput::new(
            hai,
            "0",
            has.as_str(),
            StepMessage::HighA).padding(15))
        .push(Text::new("b: "))
        .push(TextInput::new(
            hbi,
            "0",
            hbs.as_str(),
            StepMessage::HighB).padding(15));

    let low_row = Row::new().spacing(5)
        .padding(10)
        .align_items(Align::Center)
        .push(Text::new("L: "))
        .push(TextInput::new(
            lli,
            "0",
            lls.as_str(),
            StepMessage::LowL).padding(15))
        .push(Text::new("a: "))
        .push(TextInput::new(
            lai,
            "0",
            las.as_str(),
            StepMessage::LowA).padding(15))
        .push(Text::new("b: "))
        .push(TextInput::new(
            lbi,
            "0",
            lbs.as_str(),
            StepMessage::LowB).padding(15));

    let mut image_row = Row::new()
        .push(Image::new(format!("{}/image_chosen.png", env!("CARGO_MANIFEST_DIR"))).width(Length::Units(250)));

    if !(masked_img.is_empty()) {
        image_row = image_row.push(Image::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), masked_img)).width(Length::Units(300)));
    }

    container
        .align_items(Align::Center)
        .push(Text::new("Lower Bounds:"))
        .push(low_row)
        .push(Text::new("Upper Bounds:"))
        .push(high_row)
        .push(image_row)
        .push(Row::new().spacing(10)
            .push(Text::new("Size:"))
            .push(TextInput::new(
                sizei,
                "0.0",
                size.as_str(),
                StepMessage::Size).padding(15)))
        .push(Button::new(si, Text::new("Save")).padding(15).on_press(StepMessage::SaveHat))
}