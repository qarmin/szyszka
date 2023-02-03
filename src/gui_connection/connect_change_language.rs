use gtk4::prelude::*;
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;

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

    let lang_short = get_language_from_combo_box_text(gui_data.settings.combo_box_settings_language.active_text().unwrap().to_string()).short_text;

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_short.as_bytes()).unwrap()];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {lib} {error:?}");
        }
    }
    gui_data.update_language();
}

pub fn _load_system_language(gui_data: &GuiData) {
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Some(language) = requested_languages.get(0) {
        let old_short_lang = language.to_string();
        let mut short_lang = "".to_string();
        // removes from e.g. en_zb, ending _zd since Szyszka don't support this
        for i in old_short_lang.chars() {
            if i.is_ascii_alphabetic() {
                short_lang.push(i)
            } else {
                break;
            }
        }
        let mut found: bool = false;
        for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
            if lang.short_text == short_lang {
                found = true;
                gui_data.settings.combo_box_settings_language.set_active(Some(index as u32));
                break;
            }
        }
        if found {
            println!("INFO: Default system language {short_lang} is available, so choosing them");
        } else {
            println!("INFO: Default system language {short_lang} is not available, using English(en) instead");
        }
    }
}
