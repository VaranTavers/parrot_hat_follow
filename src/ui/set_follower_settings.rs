use iced::{Column, Text, Button, TextInput, Radio, Align};
use iced::text_input::State as TIS;
use iced::button::State as ButtonState;
use crate::ui::step_message::{StepMessage, DefaultSetting};

pub fn set_follower_settings<'a>(container: Column<'a, StepMessage>,
                                 (mcs, cts): (&String, &String),
                                 (mci, cti, si): (&'a mut TIS, &'a mut TIS, &'a mut ButtonState),
                                 ds: Option<DefaultSetting>) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Setting type:"))
            .push(DefaultSetting::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, setting| {
                    choices.push(Radio::new(
                        setting,
                        setting,
                        ds,
                        StepMessage::SettingChanged
                    ))
                },
            ))
            .push(Text::new("Minimum change to issue new command:"))
            .push(TextInput::new(
                mci,
                "0.0",
                mcs.as_str(),
                StepMessage::MinChange).padding(15))
            .push(Text::new("Center threshold:"))
            .push(TextInput::new(
                cti,
                "20.0",
                cts.as_str(),
                StepMessage::Center).padding(15)))
        .push(Button::new(si, Text::new("Save")).padding(15).on_press(StepMessage::SaveFollower))
}