use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiTrim {
    pub check_button_trim_name_start: gtk4::CheckButton,
    pub check_button_trim_name_end: gtk4::CheckButton,
    pub check_button_trim_extension_start: gtk4::CheckButton,
    pub check_button_trim_extension_end: gtk4::CheckButton,

    pub check_button_trim_case_sensitive: gtk4::CheckButton,
    pub check_button_trim_case_insensitive: gtk4::CheckButton,

    pub entry_add_text_text_to_trim: gtk4::Entry,
}

impl GuiTrim {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_trim_name_start: gtk4::CheckButton = builder.object("check_button_trim_name_start").unwrap();
        let check_button_trim_name_end: gtk4::CheckButton = builder.object("check_button_trim_name_end").unwrap();
        let check_button_trim_extension_start: gtk4::CheckButton = builder.object("check_button_trim_extension_start").unwrap();
        let check_button_trim_extension_end: gtk4::CheckButton = builder.object("check_button_trim_extension_end").unwrap();

        check_button_trim_name_end.set_group(Some(&check_button_trim_name_start));
        check_button_trim_extension_start.set_group(Some(&check_button_trim_name_start));
        check_button_trim_extension_end.set_group(Some(&check_button_trim_name_start));

        let check_button_trim_case_sensitive: gtk4::CheckButton = builder.object("check_button_trim_case_sensitive").unwrap();
        let check_button_trim_case_insensitive: gtk4::CheckButton = builder.object("check_button_trim_case_insensitive").unwrap();

        check_button_trim_case_insensitive.set_group(Some(&check_button_trim_case_sensitive));

        let entry_add_text_text_to_trim: gtk4::Entry = builder.object("entry_add_text_text_to_trim").unwrap();

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
    pub fn update_language(&self) {}
}
