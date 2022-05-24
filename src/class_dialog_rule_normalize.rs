use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiNormalize {
    pub radio_button_normalize_everything: gtk::CheckButton,
    pub radio_button_normalize_partial: gtk::CheckButton,
}

impl GuiNormalize {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_normalize_everything: gtk::CheckButton = builder.object("radio_button_normalize_everything").unwrap();
        let radio_button_normalize_partial: gtk::CheckButton = builder.object("radio_button_normalize_partial").unwrap();

        Self {
            radio_button_normalize_everything,
            radio_button_normalize_partial,
        }
    }
}
