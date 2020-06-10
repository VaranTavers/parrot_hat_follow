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
