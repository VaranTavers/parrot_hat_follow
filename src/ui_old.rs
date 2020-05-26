use iced::{Sandbox, Element, Column, Align, text_input, Row, Text, TextInput, button, Button, Image, Length};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::picture_funcs::{get_color_from_strings, mask_image};

#[derive(Debug, Clone)]
pub enum Message {
    LowL(String),
    LowA(String),
    LowB(String),
    HighL(String),
    HighA(String),
    HighB(String),
    Size(String),
    Frame(String),
    Refresh,
}

#[derive(Default)]
pub struct SettingsEditor {
    hls: String,
    has: String,
    hbs: String,
    lls: String,
    las: String,
    lbs: String,
    original_img: String,
    masked_img: String,
    send_input: button::State,
    l_low_input: text_input::State,
    a_low_input: text_input::State,
    b_low_input: text_input::State,
    l_high_input: text_input::State,
    a_high_input: text_input::State,
    b_high_input: text_input::State,
}

impl Sandbox for SettingsEditor {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("UI test")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::HighL(val) => {
                self.hls = val;
            }
            Message::HighA(val) => {
                self.has = val;
            }
            Message::HighB(val) => {
                self.hbs = val;
            }
            Message::LowL(val) => {
                self.lls = val;
            }
            Message::LowA(val) => {
                self.las = val;
            }
            Message::LowB(val) => {
                self.lbs = val;
            }
            Message::Refresh => {
                let low_result = get_color_from_strings(&self.lls, &self.las, &self.lbs);
                let high_result = get_color_from_strings(&self.hls, &self.has, &self.hbs);

                if let Ok(high) = high_result {
                    if let Ok(low) = low_result {
                        let system_time = SystemTime::now();
                        let seconds = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                        self.masked_img = format!("image{}.jpg", seconds);
                        mask_image("image.jpg", self.masked_img.as_str(), &low, &high);
                    }
                }
            }

            _ => {

            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let high_row = Row::new()
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new("L: "))
            .push(TextInput::new(
                &mut self.l_high_input,
                "",
                self.hls.as_str(),
                Message::HighL).padding(15))
            .push(Text::new("a: "))
            .push(TextInput::new(
                &mut self.a_high_input,
                "0",
                self.has.as_str(),
                Message::HighA).padding(15))
            .push(Text::new("b: "))
            .push(TextInput::new(
                &mut self.b_high_input,
                "0",
                self.hbs.as_str(),
                Message::HighB).padding(15));

        let low_row = Row::new()
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new("L: "))
            .push(TextInput::new(
                &mut self.l_low_input,
                "0",
                self.lls.as_str(),
                Message::LowL).padding(15))
            .push(Text::new("a: "))
            .push(TextInput::new(
                &mut self.a_low_input,
                "0",
                self.las.as_str(),
                Message::LowA).padding(15))
            .push(Text::new("b: "))
            .push(TextInput::new(
                &mut self.b_low_input,
                "0",
                self.lbs.as_str(),
                Message::LowB).padding(15));

        if self.original_img.is_empty() {
            self.original_img = String::from("image.jpg");
        }
        if self.masked_img.is_empty() {
            self.original_img = String::from("image2.jpg");
        }
        let image_row = Row::new()
            .push(Image::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), self.original_img)).width(Length::Units(300)))
            .push(Image::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), self.masked_img)).width(Length::Units(300)));
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(high_row)
            .push(Text::new(
                format!("({}, {}, {}), ({}, {}, {})", self.hls, self.has, self.hbs, self.lls, self.las, self.lbs).as_str()
            ))
            .push(image_row)
            .push(low_row)
            .push(Button::new(&mut self.send_input, Text::new("Send")).padding(10).on_press(Message::Refresh))
            .into()
    }
}
