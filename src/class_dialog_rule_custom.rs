use gtk::prelude::*;

#[derive(Clone)]
pub struct GUICustom {
    pub entry_custom_text_to_change: gtk::Entry,
}

impl GUICustom {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let entry_custom_text_to_change: gtk::Entry = builder.get_object("entry_custom_text_to_change").unwrap();

        Self { entry_custom_text_to_change }
    }
}
