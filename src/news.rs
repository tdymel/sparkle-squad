use comfy_i18n::i18n;
use dioxus::prelude::*;

use crate::I18n;

const LOGO: Asset = asset!("/assets/logo_black.svg");
const COURT: Asset = asset!(
    "/assets/news/court.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 160
        })
);
const SPIELTAG_25_04_27: Asset = asset!(
    "/assets/news/spieltag_25_04_27.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 201
        })
);
const STELLE_2025: Asset = asset!(
    "/assets/news/2025_stelle_tournament.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 225
        })
);
const LUETJENSEE_2025: Asset = asset!(
    "/assets/news/2025_luetjensee.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 256
        })
);
const WAKENITZ_2025: Asset = asset!(
    "/assets/news/wakenitz_2025.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 251
        })
);
const HANSE_POKAL_2025: Asset = asset!(
    "/assets/news/2025_06_Hansa_Pokal.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 320
        })
);
const EUTIN_2025: Asset = asset!(
    "/assets/news/2025_10_eutin.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 216
        })
);
const HALLOWEEN_2025: Asset = asset!(
    "/assets/news/2025_11_01_spooky.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 243
        })
);
const A3_2025_1: Asset = asset!(
    "/assets/news/2025_11_16_a3_spieltag.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 216
        })
);
const A3_2025_2: Asset = asset!(
    "/assets/news/2025_11_30_a3_2_spieltag.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 210
        })
);
const B1_2025_2: Asset = asset!(
    "/assets/news/2025_12_07_b1_spieltag.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 216
        })
);
const B1_2026_3: Asset = asset!(
    "/assets/news/2026_01_18_b1_spieltag.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 230
        })
);
const A3_2026_3: Asset = asset!(
    "/assets/news/2026_01_25_a3_spieltag.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 320
        })
);
const B1_2026_4: Asset = asset!(
    "/assets/news/2026_02_01_b1_spieltag.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 237
        })
);
const A3_2026_4: Asset = asset!(
    "/assets/news/2026_02_15_a3_spieltag.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 296
        })
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
    },
    EN: {
        title: "Sparkling News",
        articles: [
            {
                title: "4 points on the 4th HVBV A3 Mixed League match day",
                date: "2026-02-15",
                date_label: "February 15, 2026",
                description: "4 points on the 4th match day of the A3 Mixed League in Hamburg!"
            },
            {
                title: "4 points on the 4th HVBV B1 Mixed League match day",
                date: "2026-02-01",
                date_label: "February 01, 2026",
                description: "4 points on the 4th match day of the B1 Mixed League in Hamburg!"
            },
            {
                title: "3 points on the 3rd HVBV A3 Mixed League match day",
                date: "2026-01-25",
                date_label: "January 25, 2026",
                description: "3 points on the 3rd match day of the A3 Mixed League in Hamburg!"
            },
            {
                title: "4 points on the 3rd HVBV B1 Mixed League match day and our new uniform",
                date: "2026-01-18",
                date_label: "January 18, 2026",
                description: "4 points on the 3rd match day of the B1 Mixed League in Hamburg! And finally, our new uniform has arrived!"
            },
            {
                title: "Two wins on the 2nd HVBV B1 Mixed League match day",
                date: "2025-12-07",
                date_label: "December 07, 2025",
                description: "Two wins on the 2nd match day of the B1 Mixed League in Hamburg!"
            },
            {
                title: "Two wins on the 2nd HVBV A3 Mixed League match day and 1st B1 match day",
                date: "2025-11-30",
                date_label: "November 30, 2025",
                description: "Two wins on the 2nd match day of the A3 and B1 Mixed League in Hamburg!"
            },
            {
                title: "Two wins on the 1st HVBV A3 Mixed League match day",
                date: "2025-11-16",
                date_label: "November 16, 2025",
                description: "Two wins in our debut in the A3 recreational mixed league in Hamburg!"
            },
            {
                title: "The Spooky Squad caused chaos at the Halloween tournament!",
                date: "2025-11-01",
                date_label: "November 01, 2025",
                description: "A truly haunted tournament — we only played draws!"
            },
            {
                title: "Victory at a mini tournament in Eutin!",
                date: "2025-08-25",
                date_label: "October 25, 2025",
                description: "We were spontaneously invited to a mini tournament in Eutin!"
            },
            {
                title: "Narrow miss at the Hamburg Volleyball Hanse Cup",
                date: "2025-06-22",
                date_label: "June 22, 2025",
                description: "Despite winning on points, we unfortunately did not qualify for the final round of the Hamburg Volleyball Hanse Cup."
            },
            {
                title: "4th place at the 1st Mixed Volleyball Wakenitz Cup in Lübeck",
                date: "2025-06-01",
                date_label: "June 01, 2025",
                description: "Surprised by the strong result, we feel confirmed in our decision to play in A3 next season. At this tournament, we were not only able to keep up, but also to beat numerous teams at state league level and above."
            },
            {
                title: "2nd place at the 1st Mixed/Hobby Volleyball Lütjensee tournament",
                date: "2025-05-24",
                date_label: "May 24, 2025",
                description: "Once again, we just narrowly missed 1st place. In the end, we lost the 3rd deciding set 13–11. Still, we are happy with our performance!"
            },
            {
                title: "2nd place at the Mixed/Hobby Volleyball closing tournament in Stelle",
                date: "2025-05-11",
                date_label: "May 11, 2025",
                description: "We only just missed first place at the Mixed/Hobby Volleyball closing tournament in Stelle!"
            },
            {
                title: "Successful first match day at the Hanse Cup!",
                date: "2025-04-27",
                date_label: "April 27, 2025",
                description: "The new mixed volleyball team from Hamburg shines in its Hanse Cup debut and wins its first two matches!"
            },
            {
                title: "At last, we can play volleyball in our own gym right in the center of Hamburg!",
                date: "2025-04-16",
                date_label: "April 16, 2025",
                description: "Sometimes you need a bit of luck too! After just about 1.5 months, we got a gym slot right in the center of Hamburg."
            },
            {
                title: "We founded Sparkle Squad!",
                date: "2025-02-28",
                date_label: "February 28, 2025",
                description: "What started as an unofficial tournament team has now become a real volleyball team. We are now looking for more players in Hamburg around Hamm!"
            }
        ]
    },
    RU: {
        title: "Сияющие новости",
        articles: [
            {
                title: "4 очка в 4-м игровом дне HVBV A3 Mixed-Runde",
                date: "2026-02-15",
                date_label: "15 февраля 2026",
                description: "4 очка в 4-м игровом дне лиги A3 Mixed-Runde в Гамбурге!"
            },
            {
                title: "4 очка в 4-м игровом дне HVBV B1 Mixed-Runde",
                date: "2026-02-01",
                date_label: "01 февраля 2026",
                description: "4 очка в 4-м игровом дне лиги B1 Mixed-Runde в Гамбурге!"
            },
            {
                title: "3 очка в 3-м игровом дне HVBV A3 Mixed-Runde",
                date: "2026-01-25",
                date_label: "25 января 2026",
                description: "3 очка в 3-м игровом дне лиги A3 Mixed-Runde в Гамбурге!"
            },
            {
                title: "4 очка в 3-м игровом дне HVBV B1 Mixed-Runde и наша новая форма",
                date: "2026-01-18",
                date_label: "18 января 2026",
                description: "4 очка в 3-м игровом дне лиги B1 Mixed-Runde в Гамбурге! А еще наконец-то приехала наша новая форма!"
            },
            {
                title: "Две победы во 2-м игровом дне HVBV B1 Mixed-Runde",
                date: "2025-12-07",
                date_label: "07 декабря 2025",
                description: "Две победы во 2-м игровом дне лиги B1 Mixed-Runde в Гамбурге!"
            },
            {
                title: "Две победы во 2-м игровом дне HVBV A3 Mixed-Runde и 1-м игровом дне B1",
                date: "2025-11-30",
                date_label: "30 ноября 2025",
                description: "Две победы во 2-м игровом дне лиг A3 и B1 Mixed-Runde в Гамбурге!"
            },
            {
                title: "Две победы в 1-м игровом дне HVBV A3 Mixed-Runde",
                date: "2025-11-16",
                date_label: "16 ноября 2025",
                description: "Две победы в нашем дебюте в любительской смешанной лиге A3 в Гамбурге!"
            },
            {
                title: "Spooky Squad устроили настоящий хаос на хэллоуинском турнире!",
                date: "2025-11-01",
                date_label: "01 ноября 2025",
                description: "По-настоящему заколдованный турнир — мы сыграли только вничью!"
            },
            {
                title: "Победа на мини-турнире в Эутине!",
                date: "2025-08-25",
                date_label: "25 октября 2025",
                description: "Нас спонтанно пригласили на мини-турнир в Эутине!"
            },
            {
                title: "Обидный вылет из Гамбургского волейбольного Hanse-Pokal",
                date: "2025-06-22",
                date_label: "22 июня 2025",
                description: "Несмотря на победу по очкам, мы, к сожалению, не смогли выйти в финальный раунд Гамбургского волейбольного Hanse-Pokal."
            },
            {
                title: "4-е место на 1-м Mixed Volleyball Wakenitz Pokal в Любеке",
                date: "2025-06-01",
                date_label: "01 июня 2025",
                description: "Мы были приятно удивлены таким результатом, и это подтвердило наше решение играть в A3 в следующем сезоне. На этом турнире мы не только держались наравне, но и смогли обыграть множество команд уровня Landesliga и выше."
            },
            {
                title: "2-е место на 1-м Mixed/Hobby Volleyball турнире в Лютьензее",
                date: "2025-05-24",
                date_label: "24 мая 2025",
                description: "Снова мы совсем немного не дотянули до 1-го места. В итоге мы проиграли 3-й решающий сет со счетом 13:11. Но все равно мы довольны своей игрой!"
            },
            {
                title: "2-е место на Mixed/Hobby Volleyball Abschluss-turnier в Штелле",
                date: "2025-05-11",
                date_label: "11 мая 2025",
                description: "Мы совсем немного не дотянули до первого места на Mixed/Hobby Volleyball Abschluss-turnier в Штелле!"
            },
            {
                title: "Успешный первый игровой день в Hanse-Pokal!",
                date: "2025-04-27",
                date_label: "27 апреля 2025",
                description: "Новая смешанная волейбольная команда из Гамбурга ярко дебютировала в Hanse-Pokal и выиграла свои первые две игры!"
            },
            {
                title: "Наконец-то мы можем играть в волейбол в собственном зале прямо в центре Гамбурга!",
                date: "2025-04-16",
                date_label: "16 апреля 2025",
                description: "Иногда нужно и немного везения! Всего примерно через 1,5 месяца мы получили время в зале прямо в центре Гамбурга."
            },
            {
                title: "Мы основали Sparkle Squad!",
                date: "2025-02-28",
                date_label: "28 февраля 2025",
                description: "Неофициальная турнирная команда превратилась в настоящую волейбольную команду. Теперь мы ищем новых игроков в Гамбурге, особенно в районе Hamm!"
            }
        ]
    }
);

#[component]
pub fn News() -> Element {
    let i18n = consume_context::<I18n>();
    let articles = i18n.news().articles().value();
    rsx! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                class: "text-3xl",
                {i18n.news().title().to_string()}
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
