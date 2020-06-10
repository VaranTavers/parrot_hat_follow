use iced::{Column, Text, Button, TextInput, Radio, Align};
use iced::text_input::State as TIS;
use iced::button::State as ButtonState;
use crate::ui::model::{StepMessage, ControllerSetting, WindSetting, PersonSetting};

pub fn set_controller_settings<'a>(container: Column<'a, StepMessage>,
                                   si: &'a mut ButtonState,
                                   (cs, ws, ps): (Option<ControllerSetting>, Option<WindSetting>, Option<PersonSetting>)
) -> Column<'a, StepMessage> {
    container
        .align_items(Align::Center)
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Controller:"))
            .push(ControllerSetting::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, setting| {
                    choices.push(Radio::new(
                        setting,
                        setting,
                        cs,
                        StepMessage::SetController
                    ))
                },
            )))
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Virtual Wind:"))
            .push(WindSetting::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, setting| {
                    choices.push(Radio::new(
                        setting,
                        setting,
                        ws,
                        StepMessage::SetWind
                    ))
                },
            )))
        .push(Column::new().align_items(Align::Start).spacing(10)
            .push(Text::new("Virtual person:"))
            .push(PersonSetting::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, setting| {
                    choices.push(Radio::new(
                        setting,
                        setting,
                        ps,
                        StepMessage::SetPerson
                    ))
                },
            )))
        .push(Button::new(si, Text::new("Save")).padding(15).on_press(StepMessage::SaveController))
}