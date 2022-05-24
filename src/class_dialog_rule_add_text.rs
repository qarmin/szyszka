use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiAddText {
    pub radio_button_add_text_before_name: gtk::CheckButton,
    pub radio_button_add_text_after_name: gtk::CheckButton,

    pub entry_add_text_text_to_add: gtk::Entry,
}

impl GuiAddText {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_add_text_before_name: gtk::CheckButton = builder.object("radio_button_add_text_before_name").unwrap();
        let radio_button_add_text_after_name: gtk::CheckButton = builder.object("radio_button_add_text_after_name").unwrap();

        let entry_add_text_text_to_add: gtk::Entry = builder.object("entry_add_text_text_to_add").unwrap();

        Self {
            radio_button_add_text_before_name,
            radio_button_add_text_after_name,
            entry_add_text_text_to_add,
        }
    }
}
