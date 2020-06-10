use iced::{Column, Text, Button, TextInput, Radio, Align};
use iced::text_input::State as TIS;
use iced::button::State as ButtonState;

use crate::ui::step_message::{StepMessage, DefaultSetting};

pub fn set_controller_settings<'a>(container: Column<'a, StepMessage>,
                                 (mcs, cts): (&String, &String),
                                 (mci, cti, si): (&'a mut TIS, &'a mut TIS, &'a mut ButtonState),
                                 ds: Option<DefaultSetting>) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Controller to use:"))
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
            )))
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Virtual Wind:"))
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
            )))
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Virtual person:"))
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
            )))
        .push(Button::new(si, Text::new("Save")).padding(15).on_press(StepMessage::SaveFollower))
}