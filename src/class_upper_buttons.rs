use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiUpperButtons {
    pub button_start_rename: gtk::Button,
    pub button_remove_selection: gtk::Button,
    pub button_add_files: gtk::Button,
    pub button_add_folders: gtk::Button,
    pub check_button_recursive_folder_search: gtk::CheckButton,
}

impl GuiUpperButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_start_rename: gtk::Button = builder.get_object("button_start_rename").unwrap();
        let button_remove_selection: gtk::Button = builder.get_object("button_remove_selection").unwrap();
        let button_add_files: gtk::Button = builder.get_object("button_add_files").unwrap();
        let button_add_folders: gtk::Button = builder.get_object("button_add_folders").unwrap();
        let check_button_recursive_folder_search: gtk::CheckButton = builder.get_object("check_button_recursive_folder_search").unwrap();

        Self {
            button_start_rename,
            button_remove_selection,
            button_add_files,
            button_add_folders,
            check_button_recursive_folder_search,
        }
    }
}
