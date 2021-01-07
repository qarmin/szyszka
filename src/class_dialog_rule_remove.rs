use gtk::prelude::*;

#[derive(Clone)]
pub struct GUIRemove {
    pub radio_button_remove_name: gtk::RadioButton,
    pub radio_button_remove_extension: gtk::RadioButton,
    pub radio_button_remove_both: gtk::RadioButton,

    pub labels_remove_example_before: gtk::Label,
    pub labels_remove_example_after: gtk::Label,
}

impl GUIRemove {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_remove_name: gtk::RadioButton = builder.get_object("radio_button_remove_name").unwrap();
        let radio_button_remove_extension: gtk::RadioButton = builder.get_object("radio_button_remove_extension").unwrap();
        let radio_button_remove_both: gtk::RadioButton = builder.get_object("radio_button_remove_both").unwrap();

        let labels_remove_example_before: gtk::Label = builder.get_object("labels_remove_example_before").unwrap();
        let labels_remove_example_after: gtk::Label = builder.get_object("labels_remove_example_after").unwrap();

        Self {
            radio_button_remove_name,
            radio_button_remove_extension,
            radio_button_remove_both,
            labels_remove_example_before,
            labels_remove_example_after,
        }
    }
}
