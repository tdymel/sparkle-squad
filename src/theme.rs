use std::sync::LazyLock;

use libero::theme::{HexColor, Sizes, Theme};

pub static THEME: LazyLock<Theme> = LazyLock::new(|| Theme {
    primary: HexColor::new(0x17_17_17),
    secondary: HexColor::new(0x5d_5d_5d),
    grey: HexColor::new(0x8a_8a_8a),
    // More headroom than the library default (4/8/12/16/20) - we kept
    // reaching for literal values (e.g. "3rem") once "xl" ran out for
    // section-level gaps.
    spacing: Sizes::new(8, 12, 20, 32, 48),
    ..Theme::DEFAULT
});
