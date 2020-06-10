#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindSetting {
    NoWind,
    PeriodicWind,
    RandomWind,
}

impl WindSetting {
    pub fn all() -> [WindSetting; 3] {
        [
            WindSetting::NoWind,
            WindSetting::PeriodicWind,
            WindSetting::RandomWind,
        ]
    }
}

impl From<WindSetting> for String {
    fn from(setting: WindSetting) -> String {
        String::from(match setting {
            WindSetting::NoWind       => "NoWind",
            WindSetting::PeriodicWind => "PeriodicWind",
            WindSetting::RandomWind   => "RandomWind",
        })
    }
}
