use gtk::prelude::*;

#[derive(Clone)]
pub struct DialogRules {
    pub notebook_choose_rule: gtk::Notebook,

    pub dialog_with_rules: gtk::Dialog,
    pub button_dialog_close: gtk::Button,
    pub button_dialog_ok: gtk::Button,

    // UpperCase/SmallCase
    pub radio_button_letters_type_uppercase: gtk::RadioButton,
    pub radio_button_letters_type_lowercase: gtk::RadioButton,

    pub radio_button_letters_usage_name: gtk::RadioButton,
    pub radio_button_letters_usage_extension: gtk::RadioButton,
    pub radio_button_letters_usage_both: gtk::RadioButton,

    pub labels_letters_example_before: gtk::Label,
    pub labels_letters_example_after: gtk::Label,
}

impl DialogRules {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_choose_rule: gtk::Notebook = builder.get_object("notebook_choose_rule").unwrap();

        let dialog_with_rules: gtk::Dialog = builder.get_object("dialog_rules").unwrap();
        let button_dialog_close: gtk::Button = builder.get_object("button_dialog_close").unwrap();
        let button_dialog_ok: gtk::Button = builder.get_object("button_dialog_ok").unwrap();

        // UpperCase/SmallCase
        let radio_button_letters_type_uppercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_uppercase").unwrap();
        let radio_button_letters_type_lowercase: gtk::RadioButton = builder.get_object("radio_button_letters_type_lowercase").unwrap();

        let radio_button_letters_usage_name: gtk::RadioButton = builder.get_object("radio_button_letters_usage_name").unwrap();
        let radio_button_letters_usage_extension: gtk::RadioButton = builder.get_object("radio_button_letters_usage_extension").unwrap();
        let radio_button_letters_usage_both: gtk::RadioButton = builder.get_object("radio_button_letters_usage_both").unwrap();

        let labels_letters_example_before: gtk::Label = builder.get_object("labels_letters_example_before").unwrap();
        let labels_letters_example_after: gtk::Label = builder.get_object("labels_letters_example_after").unwrap();

        Self {
            notebook_choose_rule,
            dialog_with_rules,
            button_dialog_close,
            button_dialog_ok,
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
