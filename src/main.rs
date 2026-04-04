use comfy_i18n::{i18n, i18n_init};
use dioxus::prelude::*;

i18n_init!(DE, EN, RU);

impl Default for I18n {
    fn default() -> Self {
        I18n::DE
    }
}

impl I18n {
    fn suffix(self) -> &'static str {
        match self {
            I18n::EN => "en",
            I18n::RU => "ru",
            I18n::DE => "de",
        }
    }

    fn from_suffix(value: &str) -> Self {
        match value {
            "ru" => I18n::RU,
            "en" => I18n::EN,
            _ => I18n::DE,
        }
    }
}

const LOGO: Asset = asset!("/assets/logo_black.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod about_us;
mod components;
mod header;
mod news;

pub use about_us::AboutUs;
pub use header::Header;
pub use news::News;
use web_sys::wasm_bindgen::JsCast;

fn main() {
    dioxus::launch(App);
}

i18n!(
    app,
    DE: {
        title: "Sparkle Squad - Mixed Volleyball Hamburg Hamm",
        description: "Turnieraktives, internationales A3/B1 Mixed-Volleyball-Team in Hamburg Hamm. Training Mittwochs 19:30 Uhr bis 22:00 Uhr."
    },
    EN: {
        title: "Sparkle Squad - Mixed Volleyball Hamburg Hamm",
        description: "Tournament-active, international A3/B1 mixed volleyball team in Hamburg Hamm. Training on Wednesdays from 7:30 PM to 10:00 PM."
    },
    RU: {
        title: "Sparkle Squad — Смешанная волейбольная команда Гамбург-Хамм",
        description: "Международная смешанная волейбольная команда уровня A3/B1 из Гамбург-Хамма, активно участвующая в турнирах. Тренировки по средам с 19:30 до 22:00."
    }
);

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/?:lang")]
    Index { lang: String },
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Index(lang: String) -> Element {
    let i18n = I18n::from_suffix(&lang);

    let html_element = web_sys::window()
        .expect("Window not found")
        .document()
        .expect("Document not found")
        .document_element()
        .expect("Document element not found")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Failed to cast to HtmlElement");

    html_element.set_attribute("lang", i18n.suffix()).unwrap();

    rsx! {
        document::Title { {i18n.app().title().to_string()} }

        document::Meta { charset: "utf-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Meta {
            name: "description",
            content: i18n.app().description().to_string()
        }

        document::Meta { property: "og:title", content: i18n.app().title().to_string() }
        document::Meta { property: "og:description", content: i18n.app().description().to_string() }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "https://sparkle-squad.de/" }
        document::Meta { property: "og:image", content: "https://sparkle-squad.de/assets/logo_black.svg" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: i18n.app().title().to_string() }
        document::Meta { name: "twitter:description", content: i18n.app().description().to_string() }
        document::Meta { name: "twitter:image", content: "https://sparkle-squad.de/assets/logo_black.svg" }

        document::Link { rel: "icon", href: LOGO }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Link { rel: "alternate", hreflang: "de", href: "https://sparkle-squad.de/de" }
        document::Link { rel: "alternate", hreflang: "en", href: "https://sparkle-squad.de/en" }
        document::Link { rel: "alternate", hreflang: "ru", href: "https://sparkle-squad.de/ru" }
        document::Link { rel: "alternate", hreflang: "x-default", href: "https://sparkle-squad.de/" }

        document::Link {
            rel: "canonical",
            href: match i18n {
                I18n::DE => "https://sparkle-squad.de/",
                I18n::EN => "https://sparkle-squad.de/en",
                I18n::RU => "https://sparkle-squad.de/ru"
            }
        }

        main {
            class: "flex flex-col max-w-5xl mx-auto gap-8",
            Header { i18n }
            AboutUs { i18n }
            News { i18n }
        }
    }
}
