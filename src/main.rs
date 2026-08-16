use comfy_i18n::i18n_init;

i18n_init!(DE, EN, RU);

mod about_us;
mod app;
mod assets;
mod header;
mod i18n;
mod news;
mod theme;

pub use app::{App, Route};

fn main() {
    dioxus::launch(App);
}
