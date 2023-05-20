use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiSizeLetters {
    pub check_button_letters_type_uppercase: gtk4::CheckButton,
    pub check_button_letters_type_lowercase: gtk4::CheckButton,

    pub check_button_letters_usage_name: gtk4::CheckButton,
    pub check_button_letters_usage_extension: gtk4::CheckButton,
    pub check_button_letters_usage_both: gtk4::CheckButton,

    pub label_letters_tool_type: gtk4::Label,
    pub label_letters_usage_type: gtk4::Label,
}

impl GuiSizeLetters {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_letters_type_uppercase: gtk4::CheckButton = builder.object("check_button_letters_type_uppercase").unwrap();
        let check_button_letters_type_lowercase: gtk4::CheckButton = builder.object("check_button_letters_type_lowercase").unwrap();

        check_button_letters_type_lowercase.set_group(Some(&check_button_letters_type_uppercase));

        let check_button_letters_usage_name: gtk4::CheckButton = builder.object("check_button_letters_usage_name").unwrap();
        let check_button_letters_usage_extension: gtk4::CheckButton = builder.object("check_button_letters_usage_extension").unwrap();
        let check_button_letters_usage_both: gtk4::CheckButton = builder.object("check_button_letters_usage_both").unwrap();

        check_button_letters_usage_extension.set_group(Some(&check_button_letters_usage_name));
        check_button_letters_usage_both.set_group(Some(&check_button_letters_usage_name));

        let label_letters_tool_type: gtk4::Label = builder.object("label_letters_tool_type").unwrap();
        let label_letters_usage_type: gtk4::Label = builder.object("label_letters_usage_type").unwrap();

        Self {
            check_button_letters_type_uppercase,
            check_button_letters_type_lowercase,
            check_button_letters_usage_name,
            check_button_letters_usage_extension,
            check_button_letters_usage_both,
            label_letters_tool_type,
            label_letters_usage_type,
        }
    }
    pub fn update_language(&self) {
        self.check_button_letters_type_uppercase.set_label(Some(&crate::fls!("check_button_letters_type_uppercase")));
        self.check_button_letters_type_lowercase.set_label(Some(&crate::fls!("check_button_letters_type_lowercase")));
        self.check_button_letters_usage_name.set_label(Some(&crate::fls!("check_button_letters_usage_name")));
        self.check_button_letters_usage_extension.set_label(Some(&crate::fls!("check_button_letters_usage_extension")));
        self.check_button_letters_usage_both.set_label(Some(&crate::fls!("check_button_letters_usage_both")));
        self.label_letters_tool_type.set_label(&crate::fls!("label_letters_tool_type"));
        self.label_letters_usage_type.set_label(&crate::fls!("label_usage_type"));
    }
}
