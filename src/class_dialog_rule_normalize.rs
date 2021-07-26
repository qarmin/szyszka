use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiNormalize {
    pub radio_button_normalize_everything: gtk::RadioButton,
    pub radio_button_normalize_partial: gtk::RadioButton,
}

impl GuiNormalize {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_normalize_everything: gtk::RadioButton = builder.object("radio_button_normalize_everything").unwrap();
        let radio_button_normalize_partial: gtk::RadioButton = builder.object("radio_button_normalize_partial").unwrap();

        Self {
            radio_button_normalize_everything,
            radio_button_normalize_partial,
        }
    }
}
