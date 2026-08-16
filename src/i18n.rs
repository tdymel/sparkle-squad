use crate::I18n;

impl Default for I18n {
    fn default() -> Self {
        I18n::DE
    }
}

impl I18n {
    pub fn suffix(self) -> &'static str {
        match self {
            I18n::EN => "en",
            I18n::RU => "ru",
            I18n::DE => "de",
        }
    }

    pub fn from_suffix(value: &str) -> Self {
        match value {
            "ru" => I18n::RU,
            "en" => I18n::EN,
            _ => I18n::DE,
        }
    }
}
