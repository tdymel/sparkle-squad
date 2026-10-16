use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Button, Flex, Icon, Text, Title},
    sx::sx,
    theme::Size,
};

use crate::{assets::LOGO, I18n};

i18n!(
    header,
    DE: {
        greeting: "Moin 👋",
        // The title runs before and after the team name, which stays the
        // same in every language and is drawn in the primary colour.
        title_before: "Wir sind ",
        title_after: "!",
        subtitle: "Mixed Volleyball Team in Hamburg Hamm",
        pitch: "Du suchst nach einem ambitionierten und sympathischen Mixed Volleyball Team in Hamburg und du hast sichere Grundlagen (A3/B1/Bezirksliga)? Dann bist du bei uns genau richtig! Komm zum Probetraining und mach dir ein Bild von uns!",
        apply_for_tryout: "Komm zum Probetraining!",
        instagram: "Folge uns auf Instagram!",
    },
    EN: {
        greeting: "Hi 👋",
        title_before: "We are ",
        title_after: "!",
        subtitle: "Mixed volleyball team in Hamburg Hamm",
        pitch: "Are you looking for an ambitious and friendly mixed volleyball team in Hamburg, and do you already have solid fundamentals (A3/B1/district league level)? Then you’ve come to the right place! Join us for a trial practice and get to know us!",
        apply_for_tryout: "Join a trial practice!",
        instagram: "Follow us on Instagram!",
    },
    RU: {
        greeting: "Привет 👋",
        title_before: "Мы — ",
        title_after: "!",
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
                gap: "xxl",
                Flex {
                    direction: "row",
                    wrap: true,
                    align: "center",
                    justify: "center",
                    gap: "xxl",
                    // A standard Icon draws the artwork as a bare stencil in the ink
                    // colour (black, white in dark mode); the default filled variant
                    // paints the whole box.
                    // It is square by default; the logo is wider than tall.
                    // On a phone the top bar's logo is enough.
                    Icon {
                        src: LOGO.to_string(),
                        variant: "standard",
                        color: "ink",
                        size: "10rem",
                        sx: sx()
                            .width("auto")
                            .aspect_ratio("2560 / 1812")
                            .display("none")
                            .breakpoint(Size::Sm, sx().display("inline-flex")),
                    }
                    Flex {
                        direction: "column",
                        gap: "sm",
                        sx: sx().flex("1").min_width("18rem"),
                        Text { component: "span", "{i18n.header().greeting()}" }
                        Title {
                            id: "header_title",
                            "{i18n.header().title_before()}"
                            Text {
                                component: "span",
                                color: "primary",
                                // Text brings body-copy type; take the heading's.
                                sx: sx()
                                    .font_size("inherit")
                                    .font_weight("inherit")
                                    .line_height("inherit"),
                                "Sparkle Squad"
                            }
                            "{i18n.header().title_after()}"
                        }
                        Title {
                            component: "h2",
                            "{i18n.header().subtitle()}"
                        }
                    }
                }
                Flex {
                    direction: "column",
                    gap: "xl",
                    Text { size: "lg", "{i18n.header().pitch()}" }
                    Flex {
                        direction: "row",
                        wrap: true,
                        gap: "lg",
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
