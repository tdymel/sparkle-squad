use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Flex, Icon, List, ListItem, Paper, SvgData, Title},
    sx::sx,
    theme::Size,
};
use pictogram_icons_lucide as lucide;

use crate::I18n;

i18n!(
    about_us,
    DE: {
        title: "Das zeichnet uns aus",
        team: {
            label: "Starkes Team",
            facts: ["Ehrgeizig, mit dem Ziel, auf hohem Level zu spielen", "Im Schnitt 25 Jahre alt", "Spieler aus aller Welt", "Vor allem Deutsch & Englisch, dazu Ukrainisch, Russisch und Griechisch"]
        },
        training: {
            label: "Gezieltes Training",
            facts: ["Jeden Mittwoch 19:30–22:00 Uhr", "Gemeinsames Aufwärmen", "Grundtechnik & Spielszenarien", "Abschlussspiel, um das Gelernte anzuwenden"]
        },
        tournaments: {
            label: "Liga & Turniere",
            facts: ["Hamburger Mixed-Runde (A3 & B1)", "Hanse-Pokal", "Zahlreiche Turniere im Umkreis von Hamburg"]
        }
    },
    EN: {
        title: "What sets us apart",
        team: {
            label: "Strong team",
            facts: ["Ambitious, aiming to play at a high level", "25 years old on average", "Players from all over the world", "Mainly German & English, plus Ukrainian, Russian and Greek"]
        },
        training: {
            label: "Focused training",
            facts: ["Every Wednesday 7:30–10:00 pm", "Warm-up together", "Basic technique & game scenarios", "Match play to apply what we’ve learned"]
        },
        tournaments: {
            label: "League & tournaments",
            facts: ["Hamburg Mixed League (A3 & B1)", "Hanse Cup", "Many tournaments in and around Hamburg"]
        }
    },
    RU: {
        title: "Что нас отличает",
        team: {
            label: "Сильная команда",
            facts: ["Амбициозные, стремимся играть на высоком уровне", "Средний возраст — 25 лет", "Игроки со всего мира", "В основном немецкий и английский, а также украинский, русский и греческий"]
        },
        training: {
            label: "Целенаправленные тренировки",
            facts: ["Каждую среду 19:30–22:00", "Совместная разминка", "Базовая техника и игровые ситуации", "Игра в конце, чтобы применить изученное"]
        },
        tournaments: {
            label: "Лига и турниры",
            facts: ["Гамбургская смешанная лига (A3 и B1)", "Hanse-Pokal", "Множество турниров в Гамбурге и окрестностях"]
        }
    }
);

#[component]
pub fn AboutUs(i18n: I18n) -> Element {
    let about_us = i18n.about_us();
    rsx! {
        section {
            aria_labelledby: "about_us",
            Flex {
                direction: "column",
                gap: "xxl",
                Title {
                    id: "about_us",
                    component: "h2",
                    "{about_us.title()}"
                }
                // Three cards side by side from `md` up, stacked below it;
                // stretched, so a row of them shares one height.
                List {
                    sx: sx()
                        .gap("lg")
                        .align_items("stretch")
                        .breakpoint(Size::Md, sx().flex_direction("row")),
                    FeatureCard {
                        icon: lucide::users::outlined,
                        label: about_us.team().label(),
                        facts: about_us.team().facts().value(),
                    }
                    FeatureCard {
                        icon: lucide::dumbbell::outlined,
                        label: about_us.training().label(),
                        facts: about_us.training().facts().value(),
                    }
                    FeatureCard {
                        icon: lucide::trophy::outlined,
                        label: about_us.tournaments().label(),
                        facts: about_us.tournaments().facts().value(),
                    }
                }
            }
        }
    }
}

/// The card icon's tile, which the heading's first line is centred on.
const ICON_SIZE: &str = "2.5rem";

/// One card: an icon beside its heading, then a checklist of what it covers.
#[component]
fn FeatureCard(
    icon: SvgData,
    label: &'static str,
    facts: &'static [&'static str],
) -> Element {
    rsx! {
        ListItem {
            sx: sx().flex("1 1 0").min_width("0"),
            Paper {
                bordered: true,
                radius: "md",
                sx: sx()
                    .height("100%")
                    .padding("xl")
                    .display("flex")
                    .flex_direction("column")
                    .gap("lg"),
                Flex {
                    direction: "row",
                    // Top, not centre: the heading wraps in a narrow card,
                    // and the icon stays with its first line.
                    align: "flex-start",
                    gap: "md",
                    sx: sx().flex_wrap("nowrap"),
                    Icon { svg: icon, variant: "tonal", color: "primary", size: ICON_SIZE, radius: "md" }
                    // Shifted down by half the gap between the icon and one
                    // line (`1lh`), so the first line sits on the icon's middle.
                    Title {
                        size: "lg",
                        component: "h3",
                        sx: sx().margin_top(format!("calc(({ICON_SIZE} - 1lh) / 2)")),
                        {label}
                    }
                }
                List {
                    icon: rsx! {
                        Icon {
                            svg: lucide::check::outlined,
                            variant: "standard",
                            color: "primary",
                            size: "1em",
                        }
                    },
                    for fact in facts.iter() {
                        ListItem { "{fact}" }
                    }
                }
            }
        }
    }
}
