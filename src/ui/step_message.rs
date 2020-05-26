#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultSetting {
    Video,
    Debug,
    Silent
}

impl DefaultSetting {
    pub fn all() -> [DefaultSetting; 3] {
        [
            DefaultSetting::Video,
            DefaultSetting::Debug,
            DefaultSetting::Silent,
        ]
    }
}

impl From<DefaultSetting> for String {
    fn from(setting: DefaultSetting) -> String {
        String::from(match setting {
            DefaultSetting::Video => "Video",
            DefaultSetting::Debug => "Debug",
            DefaultSetting::Silent => "Silent"
        })
    }
}

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
