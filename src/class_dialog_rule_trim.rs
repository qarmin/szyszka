use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiTrim {
    pub check_button_trim_name_start: gtk::CheckButton,
    pub check_button_trim_name_end: gtk::CheckButton,
    pub check_button_trim_extension_start: gtk::CheckButton,
    pub check_button_trim_extension_end: gtk::CheckButton,

    pub check_button_trim_case_sensitive: gtk::CheckButton,
    pub check_button_trim_case_insensitive: gtk::CheckButton,

    pub entry_add_text_text_to_trim: gtk::Entry,
}

impl GuiTrim {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_trim_name_start: gtk::CheckButton = builder.object("check_button_trim_name_start").unwrap();
        let check_button_trim_name_end: gtk::CheckButton = builder.object("check_button_trim_name_end").unwrap();
        let check_button_trim_extension_start: gtk::CheckButton = builder.object("check_button_trim_extension_start").unwrap();
        let check_button_trim_extension_end: gtk::CheckButton = builder.object("check_button_trim_extension_end").unwrap();

        let check_button_trim_case_sensitive: gtk::CheckButton = builder.object("check_button_trim_case_sensitive").unwrap();
        let check_button_trim_case_insensitive: gtk::CheckButton = builder.object("check_button_trim_case_insensitive").unwrap();

        let entry_add_text_text_to_trim: gtk::Entry = builder.object("entry_add_text_text_to_trim").unwrap();

        Self {
            check_button_trim_name_start,
            check_button_trim_name_end,
            check_button_trim_extension_start,
            check_button_trim_extension_end,
            check_button_trim_case_sensitive,
            check_button_trim_case_insensitive,
            entry_add_text_text_to_trim,
        }
    }
}
