use std::fs;

use crate::config::{get_language_config_path, save_dark_theme};
use crate::localizer::localizer;

#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

pub const LANGUAGES_ALL: &[Language] = &[
    Language { combo_box_text: "English", short_text: "en" },
    Language { combo_box_text: "Français (French)", short_text: "fr" },
    Language { combo_box_text: "Italiano (Italian)", short_text: "it" },
    Language { combo_box_text: "Polski (Polish)", short_text: "pl" },
    Language { combo_box_text: "Русский (Russian)", short_text: "ru" },
    Language { combo_box_text: "український (Ukrainian)", short_text: "uk" },
    Language { combo_box_text: "Česky (Czech)", short_text: "cs" },
    Language { combo_box_text: "Deutsch (German)", short_text: "de" },
    Language { combo_box_text: "やまと (Japanese)", short_text: "ja" },
    Language { combo_box_text: "Português (Portuguese)", short_text: "pt" },
    Language { combo_box_text: "简体中文 (Simplified Chinese)", short_text: "zh" },
    Language { combo_box_text: "Español (Spanish)", short_text: "es" },
    Language { combo_box_text: "Swedish (Svenska)", short_text: "sv" },
];

pub fn get_language_from_combo_box_text(combo_box_text: &str) -> &'static Language {
    LANGUAGES_ALL.iter().find(|l| l.combo_box_text == combo_box_text).unwrap_or(&LANGUAGES_ALL[0])
}

pub fn load_saved_language() -> String {
    if let Some(path) = get_language_config_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    "English".to_string()
}

pub fn save_language(combo_text: &str) {
    if let Some(path) = get_language_config_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, combo_text);
    }
}

pub fn apply_language(combo_text: &str) {
    let lang = get_language_from_combo_box_text(combo_text);
    let localizer = localizer();
    let requested = vec![lang.short_text.parse().expect("Invalid language identifier")];
    let _ = localizer.select(&requested);
}

#[allow(dead_code)]
pub fn save_dark_theme_setting(is_dark: bool) {
    save_dark_theme(is_dark);
}
