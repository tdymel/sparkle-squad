use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Box, Button, Flex, Select, Text, ThemeSwitcher, Title},
    sx::sx,
};

use crate::{I18n, Route};

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
pub fn Header(i18n: I18n) -> Element {
    rsx! {
        section {
            aria_labelledby: "header_title",
            Flex {
                direction: "column",
                gap: "xl",
                Flex {
                    direction: "row",
                    wrap: true,
                    align: "center",
                    justify: "center",
                    gap: "xl",
                    // The logo is a mask over the ink colour rather than an
                    // `<img>`, so the black artwork turns light in dark mode.
                    Box {
                        aria_hidden: "true",
                        sx: sx()
                            .height("10rem")
                            .aspect_ratio("2560 / 1812")
                            .flex_shrink("0")
                            .background_color("var(--lsx-ink)")
                            .mask(format!("url({LOGO}) center / contain no-repeat")),
                    }
                    Flex {
                        direction: "column",
                        gap: "xs",
                        sx: sx().flex("1").min_width("18rem"),
                        Flex {
                            direction: "row",
                            align: "center",
                            gap: "sm",
                            Text { component: "span", "{i18n.header().greeting()}" }
                            LanguageSwitch { i18n: i18n }
                            ThemeSwitcher {}
                        }
                        Title {
                            id: "header_title",
                            "{i18n.header().title()}"
                        }
                        Title {
                            component: "h2",
                            "{i18n.header().subtitle()}"
                        }
                    }
                }
                Flex {
                    direction: "column",
                    gap: "lg",
                    Text { size: "lg", "{i18n.header().pitch()}" }
                    Flex {
                        direction: "row",
                        wrap: true,
                        gap: "md",
                        Button {
                            variant: "outlined",
                            to: "https://forms.gle/migmPaXywrYJzXEx9",
                            target: "_blank",
                            new_tab_icon: true,
                            sx: sx().flex("1").min_width("18rem"),
                            "{i18n.header().apply_for_tryout()}"
                        }
                        Button {
                            variant: "filled",
                            to: "https://www.instagram.com/sparklesquad_team",
                            target: "_blank",
                            new_tab_icon: true,
                            sx: sx().flex("1").min_width("18rem"),
                            "{i18n.header().instagram()}"
                        }
                    }
                }
            }
        }
    }
}

i18n!(
    language_switch,
    DE: {
        switch_label: "Sprache auswählen",
        label: "Deutsch"
    },
    EN: {
        switch_label: "Select language",
        label: "English"
    },
    RU: {
        switch_label: "Выбрать язык",
        label: "Русский"
    },
);

#[component]
pub fn LanguageSwitch(i18n: I18n) -> Element {
    let nav = use_navigator();

    rsx! {
        Select {
            sx: sx().margin_left("auto"),
            value: Some(i18n),
            aria_label: *i18n.language_switch().switch_label(),
            onchange: move |value: Option<I18n>| {
                if let Some(value) = value {
                    nav.push(Route::Index { lang: value.suffix().to_string() });
                }
            },
        }
    }
}
