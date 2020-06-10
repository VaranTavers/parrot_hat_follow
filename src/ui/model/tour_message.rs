use crate::ui::model::StepMessage;

#[derive(Debug, Clone)]
pub enum TourMessage {
    StartPressed,
    BackPressed,
    NextPressed,
    EndPressed,
    StepMessage(StepMessage),
}
