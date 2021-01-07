use gtk::prelude::*;

#[derive(Clone)]
pub struct GUIStatus {
    pub label_status: gtk::Label,
}

impl GUIStatus {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let label_status: gtk::Label = builder.get_object("label_status").unwrap();

        Self { label_status }
    }
}
