use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiUpperButtons {
    pub button_start_rename: gtk::Button,
    pub button_remove_selection: gtk::Button,
    pub button_add_files: gtk::Button,
    pub button_add_folders: gtk::Button,
}

impl GuiUpperButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_start_rename: gtk::Button = builder.object("button_start_rename").unwrap();
        let button_remove_selection: gtk::Button = builder.object("button_remove_selection").unwrap();
        let button_add_files: gtk::Button = builder.object("button_add_files").unwrap();
        let button_add_folders: gtk::Button = builder.object("button_add_folders").unwrap();

        Self {
            button_start_rename,
            button_remove_selection,
            button_add_files,
            button_add_folders,
        }
    }
}
