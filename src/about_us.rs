use comfy_i18n::i18n;
use dioxus::prelude::*;

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
    }
);

#[component]
pub fn AboutUs() -> Element {
    rsx! {
         div {
            class: "flex flex-col gap-4",
            h2 {
                class: "text-3xl",
                {I18n::DE.header().about_us().title().to_string()}
            }
            dl {
                class: "flex flex-col",
                description_item {
                    label: I18n::DE.header().about_us().team().label(),
                    description: I18n::DE.header().about_us().team().description()
                }
                div { class: "divider" }
                description_item {
                    label: I18n::DE.header().about_us().training().label(),
                    description: I18n::DE.header().about_us().training().description()
                }
                div { class: "divider" }
                description_item {
                    label: I18n::DE.header().about_us().tournaments().label(),
                    description: I18n::DE.header().about_us().tournaments().description()
                }
            }
        }
    }
}

#[component]
fn description_item(label: &'static str, description: &'static str) -> Element {
    rsx! {
        div {
            div {
                class: "flex flex-col gap-2",
                aria_hidden: true,
                h3 {
                    class: "text-2xl",
                    {label}
                }
                span {
                    class: "text-lg",
                    {description}
                }
            }
            dt { class: "sr-only", {label}}
            dd { class: "sr-only", {description}}
        }
    }
}
