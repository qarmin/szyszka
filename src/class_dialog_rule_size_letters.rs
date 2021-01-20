use gtk::prelude::*;

#[derive(Clone)]
pub struct GUISizeLetters {
    pub radio_button_letters_type_uppercase: gtk::RadioButton,
    pub radio_button_letters_type_lowercase: gtk::RadioButton,

    pub radio_button_letters_usage_name: gtk::RadioButton,
    pub radio_button_letters_usage_extension: gtk::RadioButton,
    pub radio_button_letters_usage_both: gtk::RadioButton,
}

impl GUISizeLetters {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_letters_type_uppercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_uppercase").unwrap();
        let radio_button_letters_type_lowercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_lowercase").unwrap();

        let radio_button_letters_usage_name: gtk::RadioButton = builder.get_object("radio_button_letters_usage_name").unwrap();
        let radio_button_letters_usage_extension: gtk::RadioButton = builder.get_object("radio_button_letters_usage_extension").unwrap();
        let radio_button_letters_usage_both: gtk::RadioButton = builder.get_object("radio_button_letters_usage_both").unwrap();

        Self {
            radio_button_letters_type_uppercase,
            radio_button_letters_type_lowercase,
            radio_button_letters_usage_name,
            radio_button_letters_usage_extension,
            radio_button_letters_usage_both,
        }
    }
}
