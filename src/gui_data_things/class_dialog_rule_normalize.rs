use gtk4::prelude::*;
use gtk4::CheckButton;

#[derive(Clone)]
pub struct GuiNormalize {
    pub check_button_normalize_everything: CheckButton,
    pub check_button_normalize_partial: CheckButton,
}

impl GuiNormalize {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_normalize_everything: CheckButton = builder.object("check_button_normalize_everything").unwrap();
        let check_button_normalize_partial: CheckButton = builder.object("check_button_normalize_partial").unwrap();

        check_button_normalize_partial.set_group(Some(&check_button_normalize_everything));

        Self {
            check_button_normalize_everything,
            check_button_normalize_partial,
        }
    }
    pub fn update_language(&self) {}
}
