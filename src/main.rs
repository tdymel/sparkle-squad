use comfy_i18n::{i18n, i18n_init};
use dioxus::prelude::*;

i18n_init!(DE, EN, RU);

const LOGO: Asset = asset!("/assets/logo_black.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod header;
mod news;
mod about_us;

pub use header::Header;
pub use news::News;
pub use about_us::AboutUs;

fn main() {
    dioxus::launch(App);
}

i18n!(
    app,
    DE: {
        title: "Sparkle Squad - Mixed Volleyball Hamburg Hamm"
    }
);

#[component]
fn App() -> Element {
    rsx! {
        document::Title { {I18n::DE.app().title().to_string()} }
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
