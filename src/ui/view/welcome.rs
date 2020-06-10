use iced::{Column, Text};

use crate::ui::model::StepMessage;

pub fn welcome<'a>(container: Column<'a, StepMessage>) -> Column<'a, StepMessage> {
    container
        .push(Text::new("Welcome! You can use the following tabs to configure the algorithm..."))
        .push(Text::new("... or you can dash through them and use the default settings!"))
        .push(Text::new("The choice is yours."))
}