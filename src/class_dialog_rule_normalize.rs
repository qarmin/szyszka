use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiNormalize {
    pub check_button_normalize_everything: gtk::CheckButton,
    pub check_button_normalize_partial: gtk::CheckButton,
}

impl GuiNormalize {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_normalize_everything: gtk::CheckButton = builder.object("check_button_normalize_everything").unwrap();
        let check_button_normalize_partial: gtk::CheckButton = builder.object("check_button_normalize_partial").unwrap();

        Self {
            check_button_normalize_everything,
            check_button_normalize_partial,
        }
    }
}
