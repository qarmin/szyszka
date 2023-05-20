use gtk4::prelude::*;

use crate::fls;

#[derive(Clone)]
pub struct GuiAddText {
    pub check_button_add_text_before_name: gtk4::CheckButton,
    pub check_button_add_text_after_name: gtk4::CheckButton,

    pub label_add_text: gtk4::Label,

    pub entry_add_text_text_to_add: gtk4::Entry,
}

impl GuiAddText {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_add_text_before_name: gtk4::CheckButton = builder.object("check_button_add_text_before_name").unwrap();
        let check_button_add_text_after_name: gtk4::CheckButton = builder.object("check_button_add_text_after_name").unwrap();

        check_button_add_text_before_name.set_group(Some(&check_button_add_text_after_name));

        let entry_add_text_text_to_add: gtk4::Entry = builder.object("entry_add_text_text_to_add").unwrap();
        let label_add_text: gtk4::Label = builder.object("label_add_text").unwrap();
        Self {
            check_button_add_text_before_name,
            check_button_add_text_after_name,
            label_add_text,
            entry_add_text_text_to_add,
        }
    }
    pub fn update_language(&self) {
        self.label_add_text.set_text(&fls!("label_add_text"));
        self.check_button_add_text_after_name.set_label(Some(&fls!("check_button_add_text_after_name")));
        self.check_button_add_text_before_name.set_label(Some(&fls!("check_button_add_text_before_name")));
    }
}
