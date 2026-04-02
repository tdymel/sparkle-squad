use comfy_i18n::{i18n, i18n_init};
use dioxus::prelude::*;

i18n_init!(DE, EN, RU);

const LOGO: Asset = asset!("/assets/logo_black.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod about_us;
mod header;
mod news;

pub use about_us::AboutUs;
pub use header::Header;
pub use news::News;

fn main() {
    dioxus::launch(App);
}

i18n!(
    app,
    DE: {
        title: "Sparkle Squad - Mixed Volleyball Hamburg Hamm"
    },
    EN: {
        title: "Sparkle Squad - Mixed Volleyball Hamburg Hamm"
    },
    RU: {
        title: "Sparkle Squad — микс-волейбол, Гамбург-Хамм"
    },
);

#[component]
fn App() -> Element {
    let path = web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/".to_string());

    let i18n = i18n_from_path(&path);

    use_context_provider(|| i18n);

    rsx! {
        document::Title { {i18n.app().title().to_string()} }
        document::Link { rel: "icon", href: LOGO }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex flex-col max-w-5xl mx-auto gap-8",
            Header {}
            AboutUs {}
            News {}
        }
    }
}

fn i18n_from_path(path: &str) -> I18n {
    match path.trim_end_matches('/').rsplit('/').next() {
        Some("ru") => I18n::RU,
        Some("en") => I18n::EN,
        _ => I18n::DE,
    }
}
