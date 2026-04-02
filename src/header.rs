use comfy_i18n::i18n;
use dioxus::prelude::*;

use crate::I18n;

const LOGO: Asset = asset!("/assets/logo_black.svg");

i18n!(
    header,
    DE: {
        greeting: "Moin 👋",
        title: "Wir sind Sparkle Squad!",
        subtitle: "Mixed Volleyball Team in Hamburg Hamm",
        pitch: "Du suchst nach einem ambitionierten und sympathischen Mixed Volleyball Team in Hamburg und du hast sichere Grundlagen (A3/B1/Bezirksliga)? Dann bist du bei uns genau richtig! Komm zum Probetraining und mach dir ein Bild von uns!",
        apply_for_tryout: "Komm zum Probetraining!",
        instagram: "Folge uns auf Instagram!",
    },
    EN: {
        greeting: "Hi 👋",
        title: "We are Sparkle Squad!",
        subtitle: "Mixed volleyball team in Hamburg Hamm",
        pitch: "Are you looking for an ambitious and friendly mixed volleyball team in Hamburg, and do you already have solid fundamentals (A3/B1/district league level)? Then you’ve come to the right place! Join us for a trial practice and get to know us!",
        apply_for_tryout: "Join a trial practice!",
        instagram: "Follow us on Instagram!",
    },
    RU: {
        greeting: "Привет 👋",
        title: "Мы — Sparkle Squad!",
        subtitle: "Смешанная волейбольная команда в Гамбурге, район Hamm",
        pitch: "Ты ищешь амбициозную и дружелюбную смешанную волейбольную команду в Гамбурге и уже уверенно владеешь базовыми навыками (уровень A3/B1/Bezirksliga)? Тогда тебе точно к нам! Приходи на пробную тренировку и познакомься с нами!",
        apply_for_tryout: "Приходи на пробную тренировку!",
        instagram: "Подписывайся на нас в Instagram!",
    }
);

#[component]
pub fn Header() -> Element {
    let i18n = consume_context::<I18n>();

    rsx! {
        div {
            class: "flex flex-col gap-8",
            div {
                class: "flex flex-row items-center gap-8 flex-wrap",
                img {
                    class: "h-[10rem] mx-auto",
                    src: LOGO
                }
                div {
                    class: "flex flex-col flex-1",
                    div {
                        class: "flex flex-row",
                        span {
                            class: "text-xl py-1",
                            {i18n.header().greeting().to_string()}
                        }
                        LanguageSwitch { i18n: i18n }
                    }
                    h1 {
                        class: "text-5xl",
                        {i18n.header().title().to_string()}
                    }
                    h2 {
                        class: "text-3xl py-3",
                        {i18n.header().subtitle().to_string()}
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                span {
                    class: "text-lg",
                    {i18n.header().pitch().to_string()}
                }
                div {
                    class: "flex flex-row gap-4 flex-wrap",
                    a {
                        class: "btn btn-outline flex-1 min-w-[18rem]",
                        href: "https://forms.gle/3CNuhfGuEmAFBc858",
                        target: "_blank",
                        {i18n.header().apply_for_tryout().to_string()}
                    }
                    a {
                        class: "btn flex-1 min-w-[18rem]",
                        href: "https://www.instagram.com/sparklesquad_team",
                        target: "_blank",
                        {i18n.header().instagram().to_string()}
                    }
                }
            }
        }
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

fn replace_lang_suffix(path: &str, lang: I18n) -> String {
    let trimmed = path.trim_end_matches('/');
    let parts: Vec<&str> = trimmed.split('/').filter(|p| !p.is_empty()).collect();

    let new_parts = if let Some(last) = parts.last() {
        if matches!(*last, "en" | "ru" | "de") {
            let mut p = parts;
            let len = p.len();
            p[len - 1] = lang.suffix();
            p
        } else {
            let mut p = parts;
            p.push(lang.suffix());
            p
        }
    } else {
        vec![lang.suffix()]
    };

    format!("/{}", new_parts.join("/"))
}

i18n!(
    language_switch,
    DE: {
        label: "Deutsch"
    },
    EN: {
        label: "English"
    },
    RU: {
        label: "Русский"
    },
);

#[component]
pub fn LanguageSwitch(i18n: I18n) -> Element {
    rsx! {
        select {
            class: "select border-0 outline-none focus:outline-none focus:ring-0 shadow-none max-w-[7rem] ml-auto",
            value: i18n.suffix(),
            onchange: move |evt| {
                let new_lang = I18n::from_suffix(&evt.value());
                if let Some(window) = web_sys::window() {
                    let location = window.location();
                    let path = location.pathname().unwrap_or_else(|_| "/".to_string());
                    let new_path = replace_lang_suffix(&path, new_lang);
                    let _ = location.set_href(&new_path);
                }
            },

            option { value: "de", {I18n::DE.language_switch().label().to_string()} }
            option { value: "en", {I18n::EN.language_switch().label().to_string()} }
            option { value: "ru", {I18n::RU.language_switch().label().to_string()} }
        }
    }
}
