use crate::fl;
use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiUpperButtons {
    pub button_setting: gtk::Button,
    pub button_start_rename: gtk::Button,
    pub button_remove_selection: gtk::Button,
    pub button_add_files: gtk::Button,
    pub button_add_folders: gtk::Button,
    pub button_select_popup: gtk::Button,
    pub button_update_names: gtk::Button,
    pub label_files_folders: gtk::Label,
    pub button_results_one_up: gtk::Button,
    pub button_results_one_down: gtk::Button,
}

impl GuiUpperButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_setting: gtk::Button = builder.object("button_setting").unwrap();
        let button_start_rename: gtk::Button = builder.object("button_start_rename").unwrap();
        let button_remove_selection: gtk::Button = builder.object("button_remove_selection").unwrap();
        let button_add_files: gtk::Button = builder.object("button_add_files").unwrap();
        let button_add_folders: gtk::Button = builder.object("button_add_folders").unwrap();
        let button_select_popup: gtk::Button = builder.object("button_select_popup").unwrap();
        let button_update_names: gtk::Button = builder.object("button_update_names").unwrap();
        let label_files_folders: gtk::Label = builder.object("label_files_folders").unwrap();
        let button_results_one_up: gtk::Button = builder.object("button_results_one_up").unwrap();
        let button_results_one_down: gtk::Button = builder.object("button_results_one_down").unwrap();

        Self {
            button_setting,
            button_start_rename,
            button_remove_selection,
            button_add_files,
            button_add_folders,
            button_select_popup,
            button_update_names,
            label_files_folders,
            button_results_one_up,
            button_results_one_down,
        }
    }
    pub fn update_language(&self) {
        self.button_start_rename.set_label(&fl!("upper_start_renaming_button"));

        // pub button_setting: gtk::Button,
        // pub button_start_rename: gtk::Button,
        // pub button_remove_selection: gtk::Button,
        // pub button_add_files: gtk::Button,
        // pub button_add_folders: gtk::Button,
        // pub button_select_popup: gtk::Button,
        // pub button_update_names: gtk::Button,
        // pub label_files_folders: gtk::Label,
        // pub button_results_one_up: gtk::Button,
        // pub button_results_one_down: gtk::Button,
        // self.check_button_music_title.set_label(&fl!("music_title_checkbox"));
        // self.check_button_music_artist.set_label(&fl!("music_artist_checkbox"));
        // self.check_button_music_album_title.set_label(&fl!("music_album_title_checkbox"));
        // self.check_button_music_album_artist.set_label(&fl!("music_album_artist_checkbox"));
        // self.check_button_music_year.set_label(&fl!("music_year_checkbox"));
        // self.check_button_music_approximate_comparison.set_label(&fl!("music_comparison_checkbox"));
        //
        // self.check_button_music_approximate_comparison
        //     .set_tooltip_text(Some(&fl!("music_comparison_checkbox_tooltip")));
    }
}
