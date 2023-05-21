use crate::fls;
use gtk4::prelude::*;
use gtk4::{Builder, CheckButton, Entry, Label};

#[derive(Clone)]
pub struct GuiReplace {
    pub check_button_replace_name: CheckButton,
    pub check_button_replace_extension: CheckButton,
    pub check_button_replace_both: CheckButton,

    pub check_button_replace_case_insensitive: CheckButton,
    pub check_button_replace_case_sensitive: CheckButton,

    pub check_button_replace_regex: CheckButton,
    pub check_button_replace_replace_all: CheckButton,
    pub label_replace_captures: Label,
    pub label_replace_captured_captures: Label,

    pub entry_replace_text_to_find: Entry,
    pub entry_replace_text_to_change: Entry,

    pub label_replace_usage_type: Label,
    pub label_replace_text_to_find: Label,
    pub label_replace_text_to_replace: Label,
    pub label_replace_replacing_strings: Label,
    pub label_replace_case_sensitivity: Label,
}

impl GuiReplace {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let check_button_replace_name: CheckButton = builder.object("check_button_replace_name").unwrap();
        let check_button_replace_extension: CheckButton = builder.object("check_button_replace_extension").unwrap();
        let check_button_replace_both: CheckButton = builder.object("check_button_replace_both").unwrap();

        check_button_replace_extension.set_group(Some(&check_button_replace_name));
        check_button_replace_both.set_group(Some(&check_button_replace_name));

        let entry_replace_text_to_find: Entry = builder.object("entry_replace_text_to_find").unwrap();
        let entry_replace_text_to_change: Entry = builder.object("entry_replace_text_to_change").unwrap();

        let check_button_replace_regex: CheckButton = builder.object("check_button_replace_regex").unwrap();
        let check_button_replace_replace_all: CheckButton = builder.object("check_button_replace_replace_all").unwrap();
        let label_replace_captures: Label = builder.object("label_replace_captures").unwrap();
        let label_replace_captured_captures: Label = builder.object("label_replace_captured_captures").unwrap();

        let check_button_replace_case_sensitive: CheckButton = builder.object("check_button_replace_case_sensitive").unwrap();
        let check_button_replace_case_insensitive: CheckButton = builder.object("check_button_replace_case_insensitive").unwrap();

        let label_replace_usage_type: Label = builder.object("label_replace_usage_type").unwrap();
        let label_replace_text_to_find: Label = builder.object("label_replace_text_to_find").unwrap();
        let label_replace_text_to_replace: Label = builder.object("label_replace_text_to_replace").unwrap();
        let label_replace_replacing_strings: Label = builder.object("label_replace_replacing_strings").unwrap();
        let label_replace_case_sensitivity: Label = builder.object("label_replace_case_sensitivity").unwrap();

        check_button_replace_case_sensitive.set_group(Some(&check_button_replace_case_insensitive));

        Self {
            check_button_replace_name,
            check_button_replace_extension,
            check_button_replace_both,
            check_button_replace_case_insensitive,
            check_button_replace_case_sensitive,
            check_button_replace_regex,
            check_button_replace_replace_all,
            label_replace_captures,
            label_replace_captured_captures,
            entry_replace_text_to_find,
            entry_replace_text_to_change,
            label_replace_usage_type,
            label_replace_text_to_find,
            label_replace_text_to_replace,
            label_replace_replacing_strings,
            label_replace_case_sensitivity,
        }
    }
    pub fn update_language(&self) {
        self.check_button_replace_name.set_label(Some(&fls!("check_button_replace_name")));
        self.check_button_replace_extension.set_label(Some(&fls!("check_button_replace_extension")));
        self.check_button_replace_both.set_label(Some(&fls!("check_button_replace_both")));
        self.check_button_replace_case_insensitive.set_label(Some(&fls!("check_button_replace_case_insensitive")));
        self.check_button_replace_case_sensitive.set_label(Some(&fls!("check_button_replace_case_sensitive")));
        self.check_button_replace_regex.set_label(Some(&fls!("check_button_replace_regex")));
        self.check_button_replace_replace_all.set_label(Some(&fls!("check_button_replace_replace_all")));
        self.label_replace_captures.set_text(&fls!("label_replace_captures"));
        self.label_replace_captured_captures.set_text(&fls!("label_replace_captured_captures"));
        self.label_replace_usage_type.set_text(&fls!("label_usage_type"));
        self.label_replace_text_to_find.set_text(&fls!("label_replace_text_to_find"));
        self.label_replace_text_to_replace.set_text(&fls!("label_replace_text_to_replace"));
        self.label_replace_replacing_strings.set_text(&fls!("label_replace_replacing_strings"));
        self.label_replace_case_sensitivity.set_text(&fls!("label_trim_case_sensitivity"));
    }
}
