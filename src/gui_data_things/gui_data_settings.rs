use gtk4::prelude::*;
use gtk4::{Builder, Button, ComboBoxText, Label, Window};

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: Window,
    pub label_settings_general_language: Label,
    pub combo_box_settings_language: ComboBoxText,
    pub button_open_rules_settings: Button,
    pub button_open_cache_custom_texts: Button,
    pub button_open_config_dir: Button,
}

impl GuiSettings {
    pub fn create_from_builder(builder: &Builder, window_main: &Window) -> Self {
        let window_settings: Window = builder.object("window_settings").unwrap();
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        let label_settings_general_language: Label = builder.object("label_settings_general_language").unwrap();
        let combo_box_settings_language: ComboBoxText = builder.object("combo_box_settings_language").unwrap();

        let button_open_rules_settings: Button = builder.object("button_open_rules_settings").unwrap();
        let button_open_cache_custom_texts: Button = builder.object("button_open_cache_custom_texts").unwrap();
        let button_open_config_dir: Button = builder.object("button_open_config_dir").unwrap();

        Self {
            window_settings,
            label_settings_general_language,
            combo_box_settings_language,
            button_open_rules_settings,
            button_open_cache_custom_texts,
            button_open_config_dir,
        }
    }
}
