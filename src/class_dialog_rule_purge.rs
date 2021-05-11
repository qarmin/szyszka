use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiPurge {
    pub radio_button_purge_name: gtk::RadioButton,
    pub radio_button_purge_extension: gtk::RadioButton,
    pub radio_button_purge_both: gtk::RadioButton,
}

impl GuiPurge {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_purge_name: gtk::RadioButton = builder.get_object("radio_button_purge_name").unwrap();
        let radio_button_purge_extension: gtk::RadioButton = builder.get_object("radio_button_purge_extension").unwrap();
        let radio_button_purge_both: gtk::RadioButton = builder.get_object("radio_button_purge_both").unwrap();

        Self {
            radio_button_purge_name,
            radio_button_purge_extension,
            radio_button_purge_both,
        }
    }
}
