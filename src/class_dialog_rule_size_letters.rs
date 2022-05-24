use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiSizeLetters {
    pub radio_button_letters_type_uppercase: gtk::CheckButton,
    pub radio_button_letters_type_lowercase: gtk::CheckButton,

    pub radio_button_letters_usage_name: gtk::CheckButton,
    pub radio_button_letters_usage_extension: gtk::CheckButton,
    pub radio_button_letters_usage_both: gtk::CheckButton,
}

impl GuiSizeLetters {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_letters_type_uppercase: gtk::CheckButton = builder.object("radio_button_letters_type_uppercase").unwrap();
        let radio_button_letters_type_lowercase: gtk::CheckButton = builder.object("radio_button_letters_type_lowercase").unwrap();

        let radio_button_letters_usage_name: gtk::CheckButton = builder.object("radio_button_letters_usage_name").unwrap();
        let radio_button_letters_usage_extension: gtk::CheckButton = builder.object("radio_button_letters_usage_extension").unwrap();
        let radio_button_letters_usage_both: gtk::CheckButton = builder.object("radio_button_letters_usage_both").unwrap();

        Self {
            radio_button_letters_type_uppercase,
            radio_button_letters_type_lowercase,
            radio_button_letters_usage_name,
            radio_button_letters_usage_extension,
            radio_button_letters_usage_both,
        }
    }
}
