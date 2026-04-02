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
        about_us: {
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
    }
);

#[component]
pub fn Header() -> Element {
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
                    span {
                        class: "text-xl py-1",
                        {I18n::DE.header().greeting().to_string()}
                    }
                    h1 {
                        class: "text-5xl",
                        {I18n::DE.header().title().to_string()}
                    }
                    h2 {
                        class: "text-3xl py-3",
                        {I18n::DE.header().subtitle().to_string()}
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                span {
                    class: "text-lg",
                    {I18n::DE.header().pitch().to_string()}
                }
                div {
                    class: "flex flex-row gap-4 flex-wrap",
                    a {
                        class: "btn btn-outline flex-1 min-w-[18rem]",
                        href: "https://forms.gle/3CNuhfGuEmAFBc858",
                        target: "_blank",
                        {I18n::DE.header().apply_for_tryout().to_string()}
                    }
                    a {
                        class: "btn flex-1 min-w-[18rem]",
                        href: "https://www.instagram.com/sparklesquad_team",
                        target: "_blank",
                        {I18n::DE.header().instagram().to_string()}
                    }
                }
            }
        }
    }
}
