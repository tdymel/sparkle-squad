use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Button, Flex, Image, Option, Select, Text, Title},
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
                    gap: "xl",
                    Image {
                        src: LOGO,
                        alt: "",
                        sx: sx().width("auto").height("10rem").flex_shrink("0"),
                    }
                    Flex {
                        direction: "column",
                        gap: "xs",
                        sx: sx().flex("1").min_width("18rem"),
                        Flex {
                            direction: "row",
                            align: "center",
                            gap: "sm",
                            Text { span: true, "{i18n.header().greeting()}" }
                            LanguageSwitch { i18n: i18n }
                        }
                        Title {
                            id: "header_title",
                            variant: "h1",
                            "{i18n.header().title()}"
                        }
                        Title {
                            variant: "h2",
                            size: "h1",
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
                            href: "https://forms.gle/migmPaXywrYJzXEx9",
                            target: "_blank",
                            sx: sx().flex("1").min_width("18rem"),
                            "{i18n.header().apply_for_tryout()}"
                        }
                        Button {
                            variant: "filled",
                            href: "https://www.instagram.com/sparklesquad_team",
                            target: "_blank",
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
            value: i18n.suffix(),
            aria_label: *i18n.language_switch().switch_label(),
            onchange: move |value: String| {
                nav.push(Route::Index { lang: value });
            },
            Option { value: "de", {I18n::DE.language_switch().label().to_string()} }
            Option { value: "en", {I18n::EN.language_switch().label().to_string()} }
            Option { value: "ru", {I18n::RU.language_switch().label().to_string()} }
        }
    }
}
