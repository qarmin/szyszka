use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiReplace {
    pub check_button_replace_name: gtk4::CheckButton,
    pub check_button_replace_extension: gtk4::CheckButton,
    pub check_button_replace_both: gtk4::CheckButton,

    pub check_button_replace_case_insensitive: gtk4::CheckButton,
    pub check_button_replace_case_sensitive: gtk4::CheckButton,

    pub entry_replace_text_to_remove: gtk4::Entry,
    pub entry_replace_text_to_change: gtk4::Entry,
}

impl GuiReplace {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_replace_name: gtk4::CheckButton = builder.object("check_button_replace_name").unwrap();
        let check_button_replace_extension: gtk4::CheckButton = builder.object("check_button_replace_extension").unwrap();
        let check_button_replace_both: gtk4::CheckButton = builder.object("check_button_replace_both").unwrap();

        check_button_replace_extension.set_group(Some(&check_button_replace_name));
        check_button_replace_both.set_group(Some(&check_button_replace_name));

        let entry_replace_text_to_remove: gtk4::Entry = builder.object("entry_replace_text_to_remove").unwrap();
        let entry_replace_text_to_change: gtk4::Entry = builder.object("entry_replace_text_to_change").unwrap();

        let check_button_replace_case_sensitive: gtk4::CheckButton = builder.object("check_button_replace_case_sensitive").unwrap();
        let check_button_replace_case_insensitive: gtk4::CheckButton = builder.object("check_button_replace_case_insensitive").unwrap();

        check_button_replace_case_sensitive.set_group(Some(&check_button_replace_case_insensitive));

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
