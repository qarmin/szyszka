#[derive(Clone)]
pub struct GuiSizeLetters {
    pub check_button_letters_type_uppercase: gtk4::CheckButton,
    pub check_button_letters_type_lowercase: gtk4::CheckButton,

    pub check_button_letters_usage_name: gtk4::CheckButton,
    pub check_button_letters_usage_extension: gtk4::CheckButton,
    pub check_button_letters_usage_both: gtk4::CheckButton,
}

impl GuiSizeLetters {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_letters_type_uppercase: gtk4::CheckButton = builder.object("check_button_letters_type_uppercase").unwrap();
        let check_button_letters_type_lowercase: gtk4::CheckButton = builder.object("check_button_letters_type_lowercase").unwrap();

        let check_button_letters_usage_name: gtk4::CheckButton = builder.object("check_button_letters_usage_name").unwrap();
        let check_button_letters_usage_extension: gtk4::CheckButton = builder.object("check_button_letters_usage_extension").unwrap();
        let check_button_letters_usage_both: gtk4::CheckButton = builder.object("check_button_letters_usage_both").unwrap();

        Self {
            check_button_letters_type_uppercase,
            check_button_letters_type_lowercase,
            check_button_letters_usage_name,
            check_button_letters_usage_extension,
            check_button_letters_usage_both,
        }
    }
}
