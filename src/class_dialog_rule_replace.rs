use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiReplace {
    pub radio_button_replace_name: gtk::RadioButton,
    pub radio_button_replace_extension: gtk::RadioButton,
    pub radio_button_replace_both: gtk::RadioButton,

    pub radio_button_replace_case_insensitive: gtk::RadioButton,
    pub radio_button_replace_case_sensitive: gtk::RadioButton,

    pub entry_replace_text_to_remove: gtk::Entry,
    pub entry_replace_text_to_change: gtk::Entry,
}

impl GuiReplace {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_replace_name: gtk::RadioButton = builder.object("radio_button_replace_name").unwrap();
        let radio_button_replace_extension: gtk::RadioButton = builder.object("radio_button_replace_extension").unwrap();
        let radio_button_replace_both: gtk::RadioButton = builder.object("radio_button_replace_both").unwrap();

        let entry_replace_text_to_remove: gtk::Entry = builder.object("entry_replace_text_to_remove").unwrap();
        let entry_replace_text_to_change: gtk::Entry = builder.object("entry_replace_text_to_change").unwrap();

        let radio_button_replace_case_sensitive: gtk::RadioButton = builder.object("radio_button_replace_case_sensitive").unwrap();
        let radio_button_replace_case_insensitive: gtk::RadioButton = builder.object("radio_button_replace_case_insensitive").unwrap();

        Self {
            radio_button_replace_name,
            radio_button_replace_extension,
            radio_button_replace_both,
            radio_button_replace_case_insensitive,
            radio_button_replace_case_sensitive,
            entry_replace_text_to_remove,
            entry_replace_text_to_change,
        }
    }
}
