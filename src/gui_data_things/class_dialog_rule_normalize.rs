use gtk4::prelude::*;
use gtk4::CheckButton;

#[derive(Clone)]
pub struct GuiNormalize {
    pub check_button_normalize_everything: CheckButton,
    pub check_button_normalize_partial: CheckButton,
    pub label_normalize_name: gtk4::Label,
}

impl GuiNormalize {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_normalize_everything: CheckButton = builder.object("check_button_normalize_everything").unwrap();
        let check_button_normalize_partial: CheckButton = builder.object("check_button_normalize_partial").unwrap();
        let label_normalize_name: gtk4::Label = builder.object("label_normalize_name").unwrap();

        check_button_normalize_partial.set_group(Some(&check_button_normalize_everything));

        Self {
            check_button_normalize_everything,
            check_button_normalize_partial,
            label_normalize_name,
        }
    }
    pub fn update_language(&self) {
        self.label_normalize_name.set_label(&crate::fls!("label_normalize_name"));
        self.check_button_normalize_everything.set_label(Some(&crate::fls!("check_button_normalize_everything")));
        self.check_button_normalize_partial.set_label(Some(&crate::fls!("check_button_normalize_partial")));
    }
}
