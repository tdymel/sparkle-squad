use comfy_i18n::i18n;
use dioxus::prelude::*;
use libero::{
    components::{Box, Container, Flex, Header, Icon, Options, Select, SelectOptionArgs, ThemeSwitcher},
    sx::sx,
    theme::{Size, ThemeSet},
};

use crate::{assets::LOGO, I18n, Route};

/// The sticky bar on top of the page: logo, language and theme.
#[component]
pub fn TopBar(i18n: I18n) -> Element {
    rsx! {
        Header {
            publish_height: true,
            glass: true,
            Container {
                size: Size::Lg,
                gutters: Size::Md,
                Flex {
                    direction: "row",
                    align: "center",
                    gap: "md",
                    sx: sx().flex_wrap("nowrap").padding_top("sm").padding_bottom("sm"),
                    // A standard Icon draws the artwork as a bare stencil in the ink
                    // colour (black, white in dark mode); the default filled variant
                    // paints the whole box.
                    // It is square by default; the logo is wider than tall.
                    Icon {
                        src: LOGO.to_string(),
                        variant: "standard",
                        color: "ink",
                        size: "2.5rem",
                        sx: sx().width("auto").aspect_ratio("2560 / 1812"),
                    }
                    Flex {
                        direction: "row",
                        align: "center",
                        gap: "sm",
                        sx: sx().flex_wrap("nowrap").margin_left("auto"),
                        LanguageSwitch { i18n }
                        // Opens on libero's own pair; the picker swaps in any
                        // of libero's other theme sets.
                        ThemeSwitcher {
                            themes: ThemeSet::CATALOGUE,
                            // Buttons and fields step through different
                            // height scales, so take the select's own.
                            size: "var(--lsx-field-height-sm)",
                        }
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
fn LanguageSwitch(i18n: I18n) -> Element {
    let nav = use_navigator();

    rsx! {
        Select {
            value: Some(i18n),
            size: "sm",
            aria_label: *i18n.language_switch().switch_label(),
            onchange: move |value: Option<I18n>| {
                if let Some(value) = value {
                    nav.push(Route::Index { lang: value.suffix().to_string() });
                }
            },
            option: |o: SelectOptionArgs<I18n>| rsx! { LanguageLabel { lang: o.value } },
            selection: |lang: I18n| rsx! { LanguageLabel { lang } },
        }
    }
}

/// A language's flag and its own name, in the trigger and in the list.
#[component]
fn LanguageLabel(lang: I18n) -> Element {
    rsx! {
        Box {
            component: "span",
            // Block-level flex, not inline-flex: inline, it sat on the value
            // slot's text baseline and drifted off the trigger's centre.
            // A line height of 1 keeps the trigger at the field's height.
            sx: sx()
                .display("flex")
                .align_items("center")
                .gap("sm")
                .line_height("1"),
            Flag { lang }
            "{lang.label()}"
        }
    }
}

/// Inline SVG rather than flag emoji, which Windows draws as letter pairs.
/// Decoration only: the name next to it is what a screen reader reads.
#[component]
fn Flag(lang: I18n) -> Element {
    rsx! {
        svg {
            "aria-hidden": "true",
            view_box: "0 0 60 40",
            width: "21",
            height: "14",
            style: "flex-shrink: 0; border-radius: 2px; box-shadow: 0 0 0 1px rgb(0 0 0 / 0.12);",
            "preserveAspectRatio": "none",
            match lang {
                I18n::DE => rsx! {
                    rect { width: "60", height: "40", fill: "#FFCE00" }
                    rect { width: "60", height: "26.67", fill: "#DD0000" }
                    rect { width: "60", height: "13.33", fill: "#000000" }
                },
                I18n::RU => rsx! {
                    rect { width: "60", height: "40", fill: "#D52B1E" }
                    rect { width: "60", height: "26.67", fill: "#0039A6" }
                    rect { width: "60", height: "13.33", fill: "#FFFFFF" }
                },
                // Union Jack, simplified: the red saltire runs centred.
                I18n::EN => rsx! {
                    rect { width: "60", height: "40", fill: "#012169" }
                    path { d: "M0,0 L60,40 M60,0 L0,40", stroke: "#FFFFFF", stroke_width: "8" }
                    path { d: "M0,0 L60,40 M60,0 L0,40", stroke: "#C8102E", stroke_width: "3" }
                    path { d: "M30,0 V40 M0,20 H60", stroke: "#FFFFFF", stroke_width: "12" }
                    path { d: "M30,0 V40 M0,20 H60", stroke: "#C8102E", stroke_width: "7" }
                },
            }
        }
    }
}
