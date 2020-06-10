#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerSetting {
    ParrotController,
    VirtualController
}

impl ControllerSetting {
    pub fn all() -> [ControllerSetting; 2] {
        [
            ControllerSetting::ParrotController,
            ControllerSetting::VirtualController,
        ]
    }
}

impl From<ControllerSetting> for String {
    fn from(setting: ControllerSetting) -> String {
        String::from(match setting {
            ControllerSetting::ParrotController => "ParrotController",
            ControllerSetting::VirtualController => "VirtualController",
        })
    }
}
