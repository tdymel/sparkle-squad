use comfy_i18n::i18n;
use dioxus::prelude::*;

use crate::I18n;

const LOGO: Asset = asset!("/assets/logo_black.svg");
const COURT: Asset = asset!("/assets/news/court.png", AssetOptions::image().with_avif());
const SPIELTAG_25_04_27: Asset = asset!(
    "/assets/news/spieltag_25_04_27.png",
    AssetOptions::image().with_avif()
);
const STELLE_2025: Asset = asset!(
    "/assets/news/2025_stelle_tournament.png",
    AssetOptions::image().with_avif()
);
const LUETJENSEE_2025: Asset = asset!(
    "/assets/news/2025_luetjensee.png",
    AssetOptions::image().with_avif()
);
const WAKENITZ_2025: Asset = asset!(
    "/assets/news/wakenitz_2025.png",
    AssetOptions::image().with_avif()
);
const HANSE_POKAL_2025: Asset = asset!(
    "/assets/news/2025_06_Hansa_Pokal.png",
    AssetOptions::image().with_avif()
);
const EUTIN_2025: Asset = asset!(
    "/assets/news/2025_10_eutin.jpg",
    AssetOptions::image().with_avif()
);
const HALLOWEEN_2025: Asset = asset!(
    "/assets/news/2025_11_01_spooky.jpg",
    AssetOptions::image().with_avif()
);
const A3_2025_1: Asset = asset!(
    "/assets/news/2025_11_16_a3_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2025_2: Asset = asset!(
    "/assets/news/2025_11_30_a3_2_spieltag.png",
    AssetOptions::image().with_avif()
);
const B1_2025_2: Asset = asset!(
    "/assets/news/2025_12_07_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const B1_2026_3: Asset = asset!(
    "/assets/news/2026_01_18_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2026_3: Asset = asset!(
    "/assets/news/2026_01_25_a3_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const B1_2026_4: Asset = asset!(
    "/assets/news/2026_02_01_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2026_4: Asset = asset!(
    "/assets/news/2026_02_15_a3_spieltag.png",
    AssetOptions::image().with_avif()
);

const THUMBNAILS: [Asset; 16] = [
    A3_2026_4,
    B1_2026_4,
    A3_2026_3,
    B1_2026_3,
    B1_2025_2,
    A3_2025_2,
    A3_2025_1,
    HALLOWEEN_2025,
    EUTIN_2025,
    HANSE_POKAL_2025,
    WAKENITZ_2025,
    LUETJENSEE_2025,
    STELLE_2025,
    SPIELTAG_25_04_27,
    COURT,
    LOGO,
];

i18n!(
    news,
    DE: {
        title: "Funkelnde Neuigkeiten",
        articles: [
            {
                title: "4 Punkte beim 4. HVBV A3 Mixed-Runde Spieltag",
                date: "2026-02-15",
                date_label: "15. Februar 2026",
                description: "4 Punkte beim 4. Spieltag in der A3 Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "4 Punkte beim 4. HVBV B1 Mixed-Runde Spieltag",
                date: "2026-02-01",
                date_label: "01. Februar 2026",
                description: "4 Punkte beim 4. Spieltag in der B1 Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "3 Punkte beim 3. HVBV A3 Mixed-Runde Spieltag",
                date: "2026-01-25",
                date_label: "25. Januar 2026",
                description: "3 Punkte beim 3. Spieltag in der A3 Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "4 Punkte beim 3. HVBV B1 Mixed-Runde Spieltag und unsere neue Uniform",
                date: "2026-01-18",
                date_label: "18. Januar 2026",
                description: "4 Punkte beim 3. Spieltag in der B1 Mixed-Runde Liga in Hamburg! Außerdem ist endlich unsere neue Uniform angekommen!"
            },
            {
                title: "Doppelsieg beim 2. HVBV B1 Mixed-Runde Spieltag",
                date: "2025-12-07",
                date_label: "07. Dezember 2025",
                description: "Doppelsieg beim 2. Spieltag in der B1 Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "Doppelsieg beim 2. HVBV A3 Mixed-Runde Spieltag und 1. B1 Spieltag",
                date: "2025-11-30",
                date_label: "30. November 2025",
                description: "Doppelsieg beim 2. Spieltag in der A3 und B1 Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "Doppelsieg beim 1. HVBV A3 Mixed-Runde Spieltag",
                date: "2025-11-16",
                date_label: "16. November 2025",
                description: "Doppelsieg beim Debut in der A3 Freizeit-Mixed-Runde Liga in Hamburg!"
            },
            {
                title: "Der Spooky Squad hat das Halloween-Turnier unsicher gemacht!",
                date: "2025-11-01",
                date_label: "01. November 2025",
                description: "Ein wirkliches verhextes Turnier, wir haben nur unentschieden gespielt!"
            },
            {
                title: "Sieg bei einem Mini-Turnier in Eutin!",
                date: "2025-08-25",
                date_label: "25. Oktober 2025",
                description: "Wir wurden spontan zu einem Mini-Turnier in Eutin eingeladen!"
            },
            {
                title: "Knappes aus beim Hamburger Volleyball Hanse-Pokal",
                date: "2025-06-22",
                date_label: "22. Juni 2025",
                description: "Trotz Gewinn nach Punkten, konnten wir uns leider nicht für die Finalrunde des Hamburger Volleyball Hanse-Pokals qualifizieren."
            },
            {
                title: "4. Platz beim 1. Mixed Volleyball Wakenitz Pokal in Lübeck",
                date: "2025-06-01",
                date_label: "01. Juni 2025",
                description: "Überrascht über das gute Ergebnis fühlen wir uns bestätigt die nächste Season in A3 zu spielen. Bei diesem Turnier konnten wir nicht nur mithalten, sondern uns auch gegen zahlreiche Teams auf Landesliga-Niveau und höher durchsetzen."
            },
            {
                title: "2. Platz beim 1. Mixed/Hobby Volleyball Lütjensee-Turnier",
                date: "2025-05-24",
                date_label: "24. Mai 2025",
                description: "Wieder haben wir ganz knapp den 1. Platz verpasst. Am Ende haben wir den 3. Entscheidungssatz 13 zu 11 verloren. Trotzdem sind wir mit unserer Leistung zufrieden!"
            },
            {
                title: "2. Platz beim Mixed/Hobby Volleyball Abschlussturnier in Stelle",
                date: "2025-05-11",
                date_label: "11. Mai 2025",
                description: "Ganz knapp haben wir den ersten Platz verpasst beim Mixed/Hobby Volleyball Abschlussturnier in Stelle!"
            },
            {
                title: "Erfolgreicher erster Spieltag beim Hanse-Pokal!",
                date: "2025-04-27",
                date_label: "27. April 2025",
                description: "Das neue Mixed Hamburger Volleyball Team glänzt beim Debut beim Hanse-Pokal und gewinnt die ersten beiden Spiele!"
            },
            {
                title: "Endlich können wir Volleyball in unserer eigenen Halle mitten in Hamburg spielen!",
                date: "2025-04-16",
                date_label: "16. April 2025",
                description: "Glück muss man auch mal haben! Nach nur knapp 1.5 Monaten haben wir eine Hallenzeit mitten in Hamburg bekommen."
            },
            {
                title: "Wir haben Sparkle Squad gegründet!",
                date: "2025-02-28",
                date_label: "28. Februar 2025",
                description: "Aus dem inoffiziellen Turnier-Team wird nun ein richtiges Volleyball Team gegründet. Jetzt suchen wir nach weiteren Spielern in Hamburg rund um Hamm!"
            }
        ]
    }
);

#[component]
pub fn News() -> Element {
    let articles = I18n::DE.news().articles().value();
    rsx! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                class: "text-3xl",
                {I18n::DE.news().title().to_string()}
            }
            for (i, article) in articles.iter().enumerate() {
                Article {
                    thumbnail: THUMBNAILS[i],
                    title: article.title(),
                    date: article.date(),
                    date_label: article.date_label(),
                    description: article.description()
                }
                if i < articles.len() - 1 {
                    div { class: "divider" }
                }
            }
        }
    }
}

#[component]
fn Article(
    thumbnail: Asset,
    title: &'static str,
    date: &'static str,
    date_label: &'static str,
    description: &'static str,
) -> Element {
    rsx! {
        article {
            class: "flex flex-row flex-wrap gap-8",
            img {
                class: "max-h-[20rem] w-[18rem] mx-auto rounded-lg",
                src: thumbnail
            }
            div {
                class: "flex flex-col flex-1 gap-4",
                div {
                    class: "flex flex-col",
                    h3 {
                        class: "text-2xl",
                        {title}
                    }
                    time {
                        datetime: date,
                        {date_label}
                    }
                }
                span {
                    class: "text-lg",
                    {description}
                }
            }
        }
    }
}
