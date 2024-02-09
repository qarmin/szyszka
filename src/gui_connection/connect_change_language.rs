use gtk4::prelude::*;
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;
use std::fs;
use std::fs::read_to_string;

use crate::config::get_language_config_path;
use crate::language_functions::{get_language_from_combo_box_text, LANGUAGES_ALL};
use crate::{localizer, GuiData};

// use i18n_embed::{DesktopLanguageRequester, Localizer};

pub fn connect_change_language(gui_data: &GuiData) {
    change_language(gui_data);

    let combo_box_settings_language = gui_data.settings.combo_box_settings_language.clone();
    let gui_data = gui_data.clone();
    combo_box_settings_language.connect_changed(move |_| {
        change_language(&gui_data);
    });
}

fn change_language(gui_data: &GuiData) {
    let localizers = vec![("szyszka", localizer::localizer())];

    let lang_short = get_language_from_combo_box_text(&gui_data.settings.combo_box_settings_language.active_text().unwrap()).short_text;

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_short.as_bytes()).unwrap()];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {lib} {error:?}");
        }
    }
    gui_data.update_language();
}

pub fn load_language(gui_data: &GuiData) {
    if let Some(short_lang) = load_settings_language() {
        if try_to_set_language(gui_data, &short_lang, true) {
            return;
        };
    };

    if let Some(short_lang) = get_system_language() {
        try_to_set_language(gui_data, &short_lang, false);
    };
}

pub fn load_settings_language() -> Option<String> {
    let Some(lang_path) = get_language_config_path() else {
        return None;
    };
    let Ok(short_lang) = read_to_string(lang_path) else {
        return None;
    };
    Some(short_lang)
}
pub fn save_language(gui_data: &GuiData) {
    let idx = gui_data.settings.combo_box_settings_language.active().unwrap_or(0) as usize;
    let lang_short = LANGUAGES_ALL[idx].short_text;

    let Some(lang_config_path) = get_language_config_path() else {
        return;
    };

    fs::write(lang_config_path, lang_short).unwrap_or_else(|e| {
        eprintln!("Error while saving language {e}");
    });
}

pub fn get_system_language() -> Option<String> {
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Some(language) = requested_languages.first() {
        let old_short_lang = language.to_string();
        let mut short_lang = String::new();
        // removes from e.g. en_zb, ending _zd since Szyszka don't support this
        for i in old_short_lang.chars() {
            if i.is_ascii_alphabetic() {
                short_lang.push(i);
            } else {
                break;
            }
        }
        return Some(short_lang);
    }
    None
}

pub fn try_to_set_language(gui_data: &GuiData, short_lang: &str, loading_from_settings: bool) -> bool {
    for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
        if lang.short_text == short_lang {
            if loading_from_settings {
                println!("INFO: Loaded language {short_lang} from settings");
            } else {
                println!("INFO: Default system language {short_lang} is available, so choosing them");
            }
            gui_data.settings.combo_box_settings_language.set_active(Some(index as u32));
            return true;
        }
    }
    if !loading_from_settings {
        println!("INFO: Default system language {short_lang} is not available, using English(en) instead");
    } else {
        println!("INFO: Failed to load language {short_lang} from settings, using English(en) instead");
    }
    false
}
