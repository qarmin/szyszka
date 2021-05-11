use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiTrim {
    pub radio_button_trim_name_start: gtk::RadioButton,
    pub radio_button_trim_name_end: gtk::RadioButton,
    pub radio_button_trim_extension_start: gtk::RadioButton,
    pub radio_button_trim_extension_end: gtk::RadioButton,

    pub radio_button_trim_case_sensitive: gtk::RadioButton,
    pub radio_button_trim_case_insensitive: gtk::RadioButton,

    pub entry_add_text_text_to_trim: gtk::Entry,
}

impl GuiTrim {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_trim_name_start: gtk::RadioButton = builder.get_object("radio_button_trim_name_start").unwrap();
        let radio_button_trim_name_end: gtk::RadioButton = builder.get_object("radio_button_trim_name_end").unwrap();
        let radio_button_trim_extension_start: gtk::RadioButton = builder.get_object("radio_button_trim_extension_start").unwrap();
        let radio_button_trim_extension_end: gtk::RadioButton = builder.get_object("radio_button_trim_extension_end").unwrap();

        let radio_button_trim_case_sensitive: gtk::RadioButton = builder.get_object("radio_button_trim_case_sensitive").unwrap();
        let radio_button_trim_case_insensitive: gtk::RadioButton = builder.get_object("radio_button_trim_case_insensitive").unwrap();

        let entry_add_text_text_to_trim: gtk::Entry = builder.get_object("entry_add_text_text_to_trim").unwrap();

        Self {
            radio_button_trim_name_start,
            radio_button_trim_name_end,
            radio_button_trim_extension_start,
            radio_button_trim_extension_end,
            radio_button_trim_case_sensitive,
            radio_button_trim_case_insensitive,
            entry_add_text_text_to_trim,
        }
    }
}
