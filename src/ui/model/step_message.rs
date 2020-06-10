use crate::ui::model::DefaultSetting;

#[derive(Debug, Clone)]
pub enum StepMessage {
    Takeoff,
    TakePicture,
    Land,
    LowL(String),
    LowA(String),
    LowB(String),
    HighL(String),
    HighA(String),
    HighB(String),
    Size(String),
    SaveHat,
    Sigma0(String),
    SigmaGain(String),
    VLose(String),
    SaveKalman,
    SettingChanged(DefaultSetting),
    MinChange(String),
    Center(String),
    SaveFollower,
    Start,
    Stop,
}
