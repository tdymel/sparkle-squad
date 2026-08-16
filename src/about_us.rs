use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Divider, Flex, List, ListItem, Text, Title},
    sx::sx,
};

use crate::I18n;

i18n!(
    about_us,
    DE: {
        title: "Das zeichnet uns aus",
        team: {
            label: "Ambitioniertes, sympathisches und internationales Team",
            description: "Wir sind ein ehrgeiziges und internationales Team, welches auf einem hohen Level spielen möchte. Im Schnitt sind wir 25 Jahre alt. Unsere Spieler kommen aus aller Welt, es wird hauptsächlich Deutsch und Englisch gesprochen, aber viele sprechen auch Ukrainisch, Russisch und Grichisch."
        },
        training: {
            label: "Strukturiertes und zielgerichtetes Training",
            description: "Jeden Mittwoch von 19:30 - 22:00 Uhr machen ein zielgerichtetes Training, um uns stetig zu verbessern. Zuerst machen wir uns gemeinsam warm. Danach verbessern wir unsere Grundtechnik und üben Spielszenarien. Am Ende spielen wir und wenden das gelernte an."
        },
        tournaments: {
            label: "Ligabetrieb & Turniere",
            description: "Wir spielen nicht nur in der Hamburger Mixed-Runde (A3 & B1) und dem Hanse-Pokal mit, sondern auch an zahlreichen anderen Turnieren im Umkreis von Hamburg."
        }
    },
    EN: {
        title: "What sets us apart",
        team: {
            label: "Ambitious, friendly and international team",
            description: "We are an ambitious and international team aiming to play at a high level. On average, we are 25 years old. Our players come from all over the world. We mainly speak German and English, but many also speak Ukrainian, Russian and Greek."
        },
        training: {
            label: "Structured and focused training",
            description: "Every Wednesday from 7:30 pm to 10:00 pm, we do focused training to keep improving steadily. First, we warm up together. Then we work on our basic technique and practice game scenarios. At the end, we play and apply what we’ve learned."
        },
        tournaments: {
            label: "League play & tournaments",
            description: "We not only compete in the Hamburg Mixed League (A3 & B1) and the Hanse Cup, but also take part in many other tournaments in and around Hamburg."
        }
    },
    RU: {
        title: "Что нас отличает",
        team: {
            label: "Амбициозная, дружелюбная и международная команда",
            description: "Мы амбициозная и международная команда, которая хочет играть на высоком уровне. В среднем нам 25 лет. Наши игроки приезжают со всего мира. В основном мы говорим на немецком и английском, но многие также говорят на украинском, русском и греческом."
        },
        training: {
            label: "Структурированные и целенаправленные тренировки",
            description: "Каждую среду с 19:30 до 22:00 у нас проходит целенаправленная тренировка, чтобы мы постоянно становились лучше. Сначала мы вместе разминаемся. Затем улучшаем базовую технику и отрабатываем игровые ситуации. В конце мы играем и применяем изученное на практике."
        },
        tournaments: {
            label: "Лига и турниры",
            description: "Мы участвуем не только в Гамбургской смешанной лиге (A3 и B1) и Hanse-Pokal, но и во многих других турнирах в Гамбурге и его окрестностях."
        }
    }
);

#[component]
pub fn AboutUs(i18n: I18n) -> Element {
    rsx! {
        section {
            aria_labelledby: "about_us",
            Flex {
                direction: "column",
                gap: "lg",
                Title {
                    id: "about_us",
                    variant: "h2",
                    size: "h1",
                    "{i18n.about_us().title()}"
                }
                List {
                    sx: sx().gap("lg"),
                    DescriptionItem {
                        label: i18n.about_us().team().label(),
                        description: i18n.about_us().team().description()
                    }
                    Divider {}
                    DescriptionItem {
                        label: i18n.about_us().training().label(),
                        description: i18n.about_us().training().description()
                    }
                    Divider {}
                    DescriptionItem {
                        label: i18n.about_us().tournaments().label(),
                        description: i18n.about_us().tournaments().description()
                    }
                }
            }
        }
    }
}

#[component]
fn DescriptionItem(label: &'static str, description: &'static str) -> Element {
    rsx! {
        ListItem {
            Flex {
                direction: "column",
                gap: "sm",
                Title { variant: "h3", size: "h2", {label} }
                Text { size: "lg", {description} }
            }
        }
    }
}
