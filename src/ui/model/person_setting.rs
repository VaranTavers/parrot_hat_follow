#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonSetting {
    StandStill,
    MoveStop,
    MoveSquares,
}

impl PersonSetting {
    pub fn all() -> [PersonSetting; 3] {
        [
            PersonSetting::StandStill,
            PersonSetting::MoveStop,
            PersonSetting::MoveSquares,
        ]
    }
}

impl From<PersonSetting> for String {
    fn from(setting: PersonSetting) -> String {
        String::from(match setting {
            PersonSetting::StandStill   => "StandStill",
            PersonSetting::MoveStop     => "MoveStop",
            PersonSetting::MoveSquares  => "MoveSquares",
        })
    }
}
