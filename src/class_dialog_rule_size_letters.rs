use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiSizeLetters {
    pub check_button_letters_type_uppercase: gtk::CheckButton,
    pub check_button_letters_type_lowercase: gtk::CheckButton,

    pub check_button_letters_usage_name: gtk::CheckButton,
    pub check_button_letters_usage_extension: gtk::CheckButton,
    pub check_button_letters_usage_both: gtk::CheckButton,
}

impl GuiSizeLetters {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_letters_type_uppercase: gtk::CheckButton = builder.object("check_button_letters_type_uppercase").unwrap();
        let check_button_letters_type_lowercase: gtk::CheckButton = builder.object("check_button_letters_type_lowercase").unwrap();

        let check_button_letters_usage_name: gtk::CheckButton = builder.object("check_button_letters_usage_name").unwrap();
        let check_button_letters_usage_extension: gtk::CheckButton = builder.object("check_button_letters_usage_extension").unwrap();
        let check_button_letters_usage_both: gtk::CheckButton = builder.object("check_button_letters_usage_both").unwrap();

        Self {
            check_button_letters_type_uppercase,
            check_button_letters_type_lowercase,
            check_button_letters_usage_name,
            check_button_letters_usage_extension,
            check_button_letters_usage_both,
        }
    }
}
