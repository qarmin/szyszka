use crate::fls;
use gtk4::prelude::*;
use gtk4::{Builder, CheckButton, Entry, Label};

#[derive(Clone)]
pub struct GuiTrim {
    pub check_button_trim_name_start: CheckButton,
    pub check_button_trim_name_end: CheckButton,
    pub check_button_trim_extension_start: CheckButton,
    pub check_button_trim_extension_end: CheckButton,

    pub check_button_trim_case_sensitive: CheckButton,
    pub check_button_trim_case_insensitive: CheckButton,

    pub entry_add_text_text_to_trim: Entry,

    pub label_trim_usage_type: Label,
    pub label_trim_case_sensitivity: Label,
    pub label_trim_trim_text: Label,
}

impl GuiTrim {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let check_button_trim_name_start: CheckButton = builder.object("check_button_trim_name_start").unwrap();
        let check_button_trim_name_end: CheckButton = builder.object("check_button_trim_name_end").unwrap();
        let check_button_trim_extension_start: CheckButton = builder.object("check_button_trim_extension_start").unwrap();
        let check_button_trim_extension_end: CheckButton = builder.object("check_button_trim_extension_end").unwrap();

        check_button_trim_name_end.set_group(Some(&check_button_trim_name_start));
        check_button_trim_extension_start.set_group(Some(&check_button_trim_name_start));
        check_button_trim_extension_end.set_group(Some(&check_button_trim_name_start));

        let check_button_trim_case_sensitive: CheckButton = builder.object("check_button_trim_case_sensitive").unwrap();
        let check_button_trim_case_insensitive: CheckButton = builder.object("check_button_trim_case_insensitive").unwrap();

        check_button_trim_case_insensitive.set_group(Some(&check_button_trim_case_sensitive));

        let entry_add_text_text_to_trim: Entry = builder.object("entry_add_text_text_to_trim").unwrap();

        let label_trim_usage_type: Label = builder.object("label_trim_usage_type").unwrap();
        let label_trim_case_sensitivity: Label = builder.object("label_trim_case_sensitivity").unwrap();
        let label_trim_trim_text: Label = builder.object("label_trim_trim_text").unwrap();

        Self {
            check_button_trim_name_start,
            check_button_trim_name_end,
            check_button_trim_extension_start,
            check_button_trim_extension_end,
            check_button_trim_case_sensitive,
            check_button_trim_case_insensitive,
            entry_add_text_text_to_trim,
            label_trim_usage_type,
            label_trim_case_sensitivity,
            label_trim_trim_text,
        }
    }
    pub fn update_language(&self) {
        self.check_button_trim_name_start.set_label(Some(&fls!("check_button_trim_name_start")));
        self.check_button_trim_name_end.set_label(Some(&fls!("check_button_trim_name_end")));
        self.check_button_trim_extension_start.set_label(Some(&fls!("check_button_trim_extension_start")));
        self.check_button_trim_extension_end.set_label(Some(&fls!("check_button_trim_extension_end")));
        self.check_button_trim_case_sensitive.set_label(Some(&fls!("check_button_trim_case_sensitive")));
        self.check_button_trim_case_insensitive.set_label(Some(&fls!("check_button_trim_case_insensitive")));

        self.label_trim_usage_type.set_label(&fls!("label_usage_type"));
        self.label_trim_case_sensitivity.set_label(&fls!("label_trim_case_sensitivity"));
        self.label_trim_trim_text.set_label(&fls!("label_trim_trim_text"));
    }
}
