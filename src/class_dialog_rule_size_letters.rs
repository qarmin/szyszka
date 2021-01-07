use gtk::prelude::*;

#[derive(Clone)]
pub struct GUISizeLetters {
    pub radio_button_letters_type_uppercase: gtk::RadioButton,
    pub radio_button_letters_type_lowercase: gtk::RadioButton,

    pub radio_button_letters_usage_name: gtk::RadioButton,
    pub radio_button_letters_usage_extension: gtk::RadioButton,
    pub radio_button_letters_usage_both: gtk::RadioButton,

    pub labels_letters_example_before: gtk::Label,
    pub labels_letters_example_after: gtk::Label,
}

impl GUISizeLetters {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_letters_type_uppercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_uppercase").unwrap();
        let radio_button_letters_type_lowercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_lowercase").unwrap();

        let radio_button_letters_usage_name: gtk::RadioButton = builder.get_object("radio_button_letters_usage_name").unwrap();
        let radio_button_letters_usage_extension: gtk::RadioButton = builder.get_object("radio_button_letters_usage_extension").unwrap();
        let radio_button_letters_usage_both: gtk::RadioButton = builder.get_object("radio_button_letters_usage_both").unwrap();

        let labels_letters_example_before: gtk::Label = builder.get_object("labels_letters_example_before").unwrap();
        let labels_letters_example_after: gtk::Label = builder.get_object("labels_letters_example_after").unwrap();

        Self {
            radio_button_letters_type_uppercase,
            radio_button_letters_type_lowercase,
            radio_button_letters_usage_name,
            radio_button_letters_usage_extension,
            radio_button_letters_usage_both,
            labels_letters_example_before,
            labels_letters_example_after,
        }
    }
}
