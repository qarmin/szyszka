use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiAddText {
    pub check_button_add_text_before_name: gtk::CheckButton,
    pub check_button_add_text_after_name: gtk::CheckButton,

    pub entry_add_text_text_to_add: gtk::Entry,
}

impl GuiAddText {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_add_text_before_name: gtk::CheckButton = builder.object("check_button_add_text_before_name").unwrap();
        let check_button_add_text_after_name: gtk::CheckButton = builder.object("check_button_add_text_after_name").unwrap();

        let entry_add_text_text_to_add: gtk::Entry = builder.object("entry_add_text_text_to_add").unwrap();

        Self {
            check_button_add_text_before_name,
            check_button_add_text_after_name,
            entry_add_text_text_to_add,
        }
    }
}
