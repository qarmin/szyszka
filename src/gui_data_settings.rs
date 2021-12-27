use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: gtk::Window,
    pub label_settings_general_language: gtk::Label,
    pub combo_box_settings_language: gtk::ComboBoxText,
}

impl GuiSettings {
    pub fn create_from_builder(builder: &gtk::Builder, window_main: &gtk::Window) -> Self {
        let window_settings: gtk::Window = builder.object("window_settings").unwrap();
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        let label_settings_general_language: gtk::Label = builder.object("label_settings_general_language").unwrap();
        let combo_box_settings_language: gtk::ComboBoxText = builder.object("combo_box_settings_language").unwrap();

        Self {
            window_settings,
            label_settings_general_language,
            combo_box_settings_language,
        }
    }
}

// fn update_language(&gui_data : GuiData){
//
// }
