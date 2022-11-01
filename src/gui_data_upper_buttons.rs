use crate::fls;
use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiUpperButtons {
    pub button_setting: gtk4::Button,
    pub button_start_rename: gtk4::Button,
    pub button_remove_selection: gtk4::Button,
    pub button_add_files: gtk4::Button,
    pub button_add_folders: gtk4::Button,
    pub menu_button_select_popup: gtk4::MenuButton,
    pub button_update_names: gtk4::Button,
    pub label_files_folders: gtk4::Label,
    pub button_results_one_up: gtk4::Button,
    pub button_results_one_down: gtk4::Button,
}

impl GuiUpperButtons {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_setting: gtk4::Button = builder.object("button_setting").unwrap();
        let button_start_rename: gtk4::Button = builder.object("button_start_rename").unwrap();
        let button_remove_selection: gtk4::Button = builder.object("button_remove_selection").unwrap();
        let button_add_files: gtk4::Button = builder.object("button_add_files").unwrap();
        let button_add_folders: gtk4::Button = builder.object("button_add_folders").unwrap();
        let menu_button_select_popup: gtk4::MenuButton = builder.object("menu_button_select_popup").unwrap();
        let button_update_names: gtk4::Button = builder.object("button_update_names").unwrap();
        let label_files_folders: gtk4::Label = builder.object("label_files_folders").unwrap();
        let button_results_one_up: gtk4::Button = builder.object("button_results_one_up").unwrap();
        let button_results_one_down: gtk4::Button = builder.object("button_results_one_down").unwrap();

        Self {
            button_setting,
            button_start_rename,
            button_remove_selection,
            button_add_files,
            button_add_folders,
            menu_button_select_popup,
            button_update_names,
            label_files_folders,
            button_results_one_up,
            button_results_one_down,
        }
    }
    pub fn update_language(&self) {
        self.button_start_rename.set_label(&fls!("upper_start_renaming_button"));

        // pub button_setting: gtk4::Button,
        // pub button_start_rename: gtk4::Button,
        // pub button_remove_selection: gtk4::Button,
        // pub button_add_files: gtk4::Button,
        // pub button_add_folders: gtk4::Button,
        // pub menu_button_select_popup: gtk4::Button,
        // pub button_update_names: gtk4::Button,
        // pub label_files_folders: gtk4::Label,
        // pub button_results_one_up: gtk4::Button,
        // pub button_results_one_down: gtk4::Button,
        // self.check_button_music_title.set_label(&fls!("music_title_checkbox"));
        // self.check_button_music_artist.set_label(&fls!("music_artist_checkbox"));
        // self.check_button_music_album_title.set_label(&fls!("music_album_title_checkbox"));
        // self.check_button_music_album_artist.set_label(&fls!("music_album_artist_checkbox"));
        // self.check_button_music_year.set_label(&fls!("music_year_checkbox"));
        // self.check_button_music_approximate_comparison.set_label(&fls!("music_comparison_checkbox"));
        //
        // self.check_button_music_approximate_comparison
        //     .set_tooltip_text(Some(&fls!("music_comparison_checkbox_tooltip")));
    }
}
