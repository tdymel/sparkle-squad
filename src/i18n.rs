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

// Lets the language switch list the languages straight off `I18n`. The value
// is the URL suffix, the label each language's own name.
impl libero::components::Options for I18n {
    fn options() -> &'static [Self] {
        &[I18n::DE, I18n::EN, I18n::RU]
    }

    fn label(&self) -> String {
        self.language_switch().label().to_string()
    }

    fn value(&self) -> String {
        self.suffix().to_string()
    }
}
