use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Container, Flex},
    sx::sx,
    theme::Size,
    LiberoProvider,
};

use crate::{about_us::AboutUs, assets::LOGO, header::Header, news::News, theme::THEME, I18n};

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
pub enum Route {
    #[route("/?:lang")]
    Index { lang: String },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Index(lang: String) -> Element {
    let i18n = I18n::from_suffix(&lang);

    // No web_sys needed for this: dioxus's own document::eval works across
    // every renderer (web, desktop, ...), unlike a raw web_sys::window() call
    // which only exists on the web target.
    use_effect(move || {
        document::eval(&format!(
            "document.documentElement.setAttribute('lang', {:?});",
            i18n.suffix()
        ));
    });

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

        LiberoProvider {
            theme: &THEME,
            main {
                Container {
                    size: Size::Lg,
                    gutters: Size::Md,
                    Flex {
                        direction: "column",
                        gap: "xl",
                        sx: sx().padding_top(Size::Xl).padding_bottom(Size::Xl),
                        Header { i18n }
                        AboutUs { i18n }
                        News { i18n }
                    }
                }
            }
        }
    }
}
