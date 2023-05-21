use std::path::Path;

use gtk4::prelude::*;

use crate::config::{create_custom_text_file_if_needed, create_rule_settings_if_needed, get_config_path, get_custom_text_config_file, get_rules_config_file};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_settings_buttons(gui_data: &GuiData) {
    let button_open_config_dir = gui_data.settings.button_open_config_dir.clone();
    let button_open_cache_custom_texts = gui_data.settings.button_open_cache_custom_texts.clone();
    let button_open_rules_settings = gui_data.settings.button_open_rules_settings.clone();

    button_open_config_dir.connect_clicked(|_f| {
        if let Some(t) = get_config_path() {
            if let Err(e) = open::that(t) {
                println!("Error opening config dir: {e}");
            }
        }
    });

    button_open_cache_custom_texts.connect_clicked(|_e| {
        if let Some(t) = get_custom_text_config_file() {
            if let Err(e) = open::that(t) {
                println!("Error opening custom text file: {e}");
            }
        }
    });

    button_open_rules_settings.connect_clicked(|_e| {
        if let Some(t) = get_rules_config_file() {
            if let Err(e) = open::that(t) {
                println!("Error opening rules config file: {e}");
            }
        }
    });

    gui_data.settings.window_settings.connect_show(move |_e| {
        create_custom_text_file_if_needed();
        create_rule_settings_if_needed();

        let config_dir_exists = get_config_path().unwrap_or(Path::new("/agasgasgas").to_path_buf()).exists();
        button_open_config_dir.set_sensitive(config_dir_exists);

        let cache_custom_text_exists = get_custom_text_config_file().unwrap_or(Path::new("/agasgasgas").to_path_buf()).exists();
        button_open_cache_custom_texts.set_sensitive(cache_custom_text_exists);

        let rules_settings_exists = get_rules_config_file().unwrap_or(Path::new("/agasgasgas").to_path_buf()).exists();
        button_open_rules_settings.set_sensitive(rules_settings_exists);
    });
}
