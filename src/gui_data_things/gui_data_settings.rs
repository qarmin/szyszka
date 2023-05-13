use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: gtk4::Window,
    pub label_settings_general_language: gtk4::Label,
    pub combo_box_settings_language: gtk4::ComboBoxText,
}

impl GuiSettings {
    pub fn create_from_builder(builder: &gtk4::Builder, window_main: &gtk4::Window) -> Self {
        let window_settings: gtk4::Window = builder.object("window_settings").unwrap();
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        let label_settings_general_language: gtk4::Label = builder.object("label_settings_general_language").unwrap();
        let combo_box_settings_language: gtk4::ComboBoxText = builder.object("combo_box_settings_language").unwrap();

        Self {
            window_settings,
            label_settings_general_language,
            combo_box_settings_language,
        }
    }
}
