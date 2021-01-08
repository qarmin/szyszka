use gtk::prelude::*;

#[derive(Clone)]
pub struct GUIPurge {
    pub radio_button_purge_name: gtk::RadioButton,
    pub radio_button_purge_extension: gtk::RadioButton,
    pub radio_button_purge_both: gtk::RadioButton,

    pub labels_purge_example_before: gtk::Label,
    pub labels_purge_example_after: gtk::Label,
}

impl GUIPurge {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let radio_button_purge_name: gtk::RadioButton = builder.get_object("radio_button_purge_name").unwrap();
        let radio_button_purge_extension: gtk::RadioButton = builder.get_object("radio_button_purge_extension").unwrap();
        let radio_button_purge_both: gtk::RadioButton = builder.get_object("radio_button_purge_both").unwrap();

        let labels_purge_example_before: gtk::Label = builder.get_object("labels_purge_example_before").unwrap();
        let labels_purge_example_after: gtk::Label = builder.get_object("labels_purge_example_after").unwrap();

        Self {
            radio_button_purge_name,
            radio_button_purge_extension,
            radio_button_purge_both,
            labels_purge_example_before,
            labels_purge_example_after,
        }
    }
}
