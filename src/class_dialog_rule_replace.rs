use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiReplace {
    pub check_button_replace_name: gtk::CheckButton,
    pub check_button_replace_extension: gtk::CheckButton,
    pub check_button_replace_both: gtk::CheckButton,

    pub check_button_replace_case_insensitive: gtk::CheckButton,
    pub check_button_replace_case_sensitive: gtk::CheckButton,

    pub entry_replace_text_to_remove: gtk::Entry,
    pub entry_replace_text_to_change: gtk::Entry,
}

impl GuiReplace {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_replace_name: gtk::CheckButton = builder.object("check_button_replace_name").unwrap();
        let check_button_replace_extension: gtk::CheckButton = builder.object("check_button_replace_extension").unwrap();
        let check_button_replace_both: gtk::CheckButton = builder.object("check_button_replace_both").unwrap();

        let entry_replace_text_to_remove: gtk::Entry = builder.object("entry_replace_text_to_remove").unwrap();
        let entry_replace_text_to_change: gtk::Entry = builder.object("entry_replace_text_to_change").unwrap();

        let check_button_replace_case_sensitive: gtk::CheckButton = builder.object("check_button_replace_case_sensitive").unwrap();
        let check_button_replace_case_insensitive: gtk::CheckButton = builder.object("check_button_replace_case_insensitive").unwrap();

        Self {
            check_button_replace_name,
            check_button_replace_extension,
            check_button_replace_both,
            check_button_replace_case_insensitive,
            check_button_replace_case_sensitive,
            entry_replace_text_to_remove,
            entry_replace_text_to_change,
        }
    }
}
