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
        section {
            class: "flex flex-col gap-8",
            aria_labelledby: "header_title",
            div {
                class: "flex flex-row items-center gap-8 flex-wrap",
                img {
                    class: "h-[10rem] w-auto mx-auto",
                    src: LOGO,
                    aria_hidden: true,
                    alt: "Logo",
                    width: "200",
                    height: "160"
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
                        id: "header_title",
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
    let nav = navigator();

    rsx! {
        select {
            class: "select border-0 outline-none focus:outline-none focus:ring-0 shadow-none max-w-[7rem] ml-auto",
            value: i18n.suffix(),
            aria_label: *i18n.language_switch().switch_label(),
            onchange: move |evt| {
                match I18n::from_suffix(&evt.value()) {
                    I18n::DE => nav.push(crate::Route::Index { i18n: I18n::DE }),
                    I18n::EN => nav.push(crate::Route::IndexEn {  }),
                    I18n::RU => nav.push(crate::Route::IndexRu {  }),
                };
            },

            option { value: "de", {I18n::DE.language_switch().label().to_string()} }
            option { value: "en", {I18n::EN.language_switch().label().to_string()} }
            option { value: "ru", {I18n::RU.language_switch().label().to_string()} }
        }
    }
}
