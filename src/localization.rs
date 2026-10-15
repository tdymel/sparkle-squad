use libero::localization::{
    AnchorLabels, CommonLabels, ImageLabels, LightboxLabels, Localization, ThemeSwitcherLabels,
};

use crate::I18n;

/// libero ships English and German only. This covers the labels of every
/// libero component the site uses; the rest stay English.
pub static RUSSIAN: Localization = Localization {
    lang: "ru",
    common: CommonLabels {
        close: "Закрыть",
        clear: "Очистить",
        remove: "Удалить {label}",
        loading: "Загрузка",
        search: "Поиск",
        warning: "Внимание:",
    },
    anchor: AnchorLabels {
        new_tab: "(откроется в новой вкладке)",
    },
    theme_switcher: ThemeSwitcherLabels {
        to_light: "Включить светлую тему",
        to_dark: "Включить тёмную тему",
        to_system: "Как в системе",
        group: "Тема",
        picker: "Выбрать тему",
        themes: "Темы",
    },
    lightbox: LightboxLabels {
        label: "Галерея",
        thumbnails: "Миниатюры",
        thumbnail: "Перейти к фото {n}",
        keys: "Z, плюс или минус для масштаба. Стрелки сдвигают увеличенное фото или листают фото. Home и End переходят к первому и последнему фото.",
        zoomed: "Масштаб {n}%",
        fitted: "Масштаб сброшен",
        zoom_in: "Увеличить",
        zoom_out: "Уменьшить",
    },
    image: ImageLabels {
        zoom: "Увеличить",
        zoom_named: "Увеличить: {alt}",
    },
    ..Localization::ENGLISH
};

/// The words libero's own components say (the lightbox, the theme switch,
/// the new-tab hint).
pub fn localization(i18n: I18n) -> &'static Localization {
    match i18n {
        I18n::DE => &Localization::GERMAN,
        I18n::EN => &Localization::ENGLISH,
        I18n::RU => &RUSSIAN,
    }
}
