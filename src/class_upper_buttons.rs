use gtk::prelude::*;

#[derive(Clone)]
pub struct UpperButtons {
    pub button_start_rename: gtk::Button,
    pub button_remove_selection: gtk::Button,
    pub button_add_entries: gtk::Button,
}

impl UpperButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_start_rename: gtk::Button = builder.get_object("button_start_rename").unwrap();
        let button_remove_selection: gtk::Button = builder.get_object("button_remove_selection").unwrap();
        let button_add_entries: gtk::Button = builder.get_object("button_add_entries").unwrap();

        Self {
            button_start_rename,
            button_remove_selection,
            button_add_entries,
        }
    }
}
