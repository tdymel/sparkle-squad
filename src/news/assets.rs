use std::collections::HashMap;
use std::sync::LazyLock;

use dioxus::prelude::*;

use crate::assets::LOGO;

const COURT: Asset = asset!(
    "/assets/news/court.png",
    ImageAssetOptions::new()
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
const SCVM_SRPING_TOURNAMENT: Asset = asset!(
    "/assets/news/2026_03_29_SCVM.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 320
        })
);
const A3_B1_VICTORY: Asset = asset!(
    "/assets/news/2026_04_12_b1_victory1.png",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 171
        })
);
const HP_2026_1: Asset = asset!(
    "/assets/news/2026_04_19_hp_1.jpg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 216
        })
);
const WAKENITZ_2026: Asset = asset!(
    "/assets/news/wakenitz_2026/urkunde.jpeg",
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Manual {
            width: 288,
            height: 320
        })
);

const COURT_FULL: Asset = asset!("/assets/news/court.png", AssetOptions::image().with_avif());
const SPIELTAG_25_04_27_FULL: Asset = asset!(
    "/assets/news/spieltag_25_04_27.png",
    AssetOptions::image().with_avif()
);
const STELLE_2025_FULL: Asset = asset!(
    "/assets/news/2025_stelle_tournament.png",
    AssetOptions::image().with_avif()
);
const LUETJENSEE_2025_FULL: Asset = asset!(
    "/assets/news/2025_luetjensee.png",
    AssetOptions::image().with_avif()
);
const WAKENITZ_2025_FULL: Asset = asset!(
    "/assets/news/wakenitz_2025.png",
    AssetOptions::image().with_avif()
);
const HANSE_POKAL_2025_FULL: Asset = asset!(
    "/assets/news/2025_06_Hansa_Pokal.png",
    AssetOptions::image().with_avif()
);
const EUTIN_2025_FULL: Asset = asset!(
    "/assets/news/2025_10_eutin.jpg",
    AssetOptions::image().with_avif()
);
const HALLOWEEN_2025_FULL: Asset = asset!(
    "/assets/news/2025_11_01_spooky.jpg",
    AssetOptions::image().with_avif()
);
const A3_2025_1_FULL: Asset = asset!(
    "/assets/news/2025_11_16_a3_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2025_2_FULL: Asset = asset!(
    "/assets/news/2025_11_30_a3_2_spieltag.png",
    AssetOptions::image().with_avif()
);
const B1_2025_2_FULL: Asset = asset!(
    "/assets/news/2025_12_07_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const B1_2026_3_FULL: Asset = asset!(
    "/assets/news/2026_01_18_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2026_3_FULL: Asset = asset!(
    "/assets/news/2026_01_25_a3_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const B1_2026_4_FULL: Asset = asset!(
    "/assets/news/2026_02_01_b1_spieltag.jpg",
    AssetOptions::image().with_avif()
);
const A3_2026_4_FULL: Asset = asset!(
    "/assets/news/2026_02_15_a3_spieltag.png",
    AssetOptions::image().with_avif()
);
const SCVM_SRPING_TOURNAMENT_FULL: Asset = asset!(
    "/assets/news/2026_03_29_SCVM.jpg",
    AssetOptions::image().with_avif()
);
const A3_B1_VICTORY_FULL: Asset = asset!(
    "/assets/news/2026_04_12_b1_victory1.png",
    AssetOptions::image().with_avif()
);
const HP_2026_1_FULL: Asset = asset!(
    "/assets/news/2026_04_19_hp_1.jpg",
    AssetOptions::image().with_avif()
);
const WAKENITZ_2026_FULL: Asset = asset!(
    "/assets/news/wakenitz_2026/urkunde.jpeg",
    AssetOptions::image().with_avif()
);

pub const THUMBNAILS: [Asset; 20] = [
    WAKENITZ_2026,
    HP_2026_1,
    A3_B1_VICTORY,
    SCVM_SRPING_TOURNAMENT,
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

pub const THUMBNAILS_FULL: [Asset; 20] = [
    WAKENITZ_2026_FULL,
    HP_2026_1_FULL,
    A3_B1_VICTORY_FULL,
    SCVM_SRPING_TOURNAMENT_FULL,
    A3_2026_4_FULL,
    B1_2026_4_FULL,
    A3_2026_3_FULL,
    B1_2026_3_FULL,
    B1_2025_2_FULL,
    A3_2025_2_FULL,
    A3_2025_1_FULL,
    HALLOWEEN_2025_FULL,
    EUTIN_2025_FULL,
    HANSE_POKAL_2025_FULL,
    WAKENITZ_2025_FULL,
    LUETJENSEE_2025_FULL,
    STELLE_2025_FULL,
    SPIELTAG_25_04_27_FULL,
    COURT_FULL,
    LOGO,
];

pub static SLIDESHOW_IMAGES: LazyLock<HashMap<&str, Vec<Asset>>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "2025-05-24",
        vec![
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_2.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_3.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_4.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_5.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_6.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/luetjensee_2025/2025_luetjensee_pokal.png",
                AssetOptions::image().with_avif()
            ),
        ],
    );

    map.insert(
        "2025-06-01",
        vec![
            asset!(
                "/assets/news/wakenitz_2025/team_bild.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/urkunde.jpg",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0252.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0540.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0541.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0542.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0575.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0591.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0632.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0641.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0904.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2025/IMG_0924.JPG",
                AssetOptions::image().with_avif()
            ),
        ],
    );

    map.insert(
        "2026-04-12",
        vec![
            asset!(
                "/assets/news/2026_04_12_b1_victory1.png",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/2026_04_12_b1_victory2.jpg",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/2026_04_12_b1_victory3.jpg",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/2026_03_22_a3_victory.png",
                AssetOptions::image().with_avif()
            ),
        ],
    );

    map.insert(
        "2026-05-02",
        vec![
            asset!(
                "/assets/news/wakenitz_2026/urkunde.jpeg",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2026/1.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2026/2.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2026/3.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2026/4.JPG",
                AssetOptions::image().with_avif()
            ),
            asset!(
                "/assets/news/wakenitz_2026/5.JPG",
                AssetOptions::image().with_avif()
            ),
        ],
    );

    map
});
