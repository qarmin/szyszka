use gtk4::prelude::*;
use gtk4::Window;

use crate::fls;
use crate::help_function::{get_custom_label_from_widget, set_icon_of_button};

pub const SZY_ICON_SETTINGS: &[u8] = include_bytes!("../../icons/szy_settings.svg");
pub const SZY_ICON_UP: &[u8] = include_bytes!("../../icons/szy_up.svg");
pub const SZY_ICON_DOWN: &[u8] = include_bytes!("../../icons/szy_down.svg");
pub const SZY_ICON_SETTINGS_DARK: &[u8] = include_bytes!("../../icons/szy_settings_dark.svg");
pub const SZY_ICON_UP_DARK: &[u8] = include_bytes!("../../icons/szy_up_dark.svg");
pub const SZY_ICON_DOWN_DARK: &[u8] = include_bytes!("../../icons/szy_down_dark.svg");

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

    pub file_chooser_dialog_add_files: gtk4::FileChooserNative,
    pub file_chooser_dialog_add_folders: gtk4::FileChooserNative,
}

impl GuiUpperButtons {
    pub fn create_from_builder(builder: &gtk4::Builder, window_main: &Window) -> Self {
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

        let file_chooser_dialog_add_files = gtk4::FileChooserNative::builder()
            .action(gtk4::FileChooserAction::Open)
            .transient_for(window_main)
            .select_multiple(true)
            .modal(true)
            .build();
        let file_chooser_dialog_add_folders = gtk4::FileChooserNative::builder()
            .action(gtk4::FileChooserAction::SelectFolder)
            .transient_for(window_main)
            .select_multiple(true)
            .modal(true)
            .build();

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
            file_chooser_dialog_add_files,
            file_chooser_dialog_add_folders,
        }
    }
    pub fn update_language(&self) {
        self.button_start_rename.set_label(&fls!("upper_start_renaming_button"));
        self.button_add_files.set_label(&fls!("upper_add_files_button"));
        self.button_add_folders.set_label(&fls!("upper_add_folders_button"));
        self.button_remove_selection.set_label(&fls!("upper_remove_selection_button"));
        self.button_update_names.set_label(&fls!("upper_update_names_button"));
        self.label_files_folders.set_label(&fls!("upper_files_folders_label"));
        get_custom_label_from_widget(&self.button_results_one_up).set_label(&fls!("upper_results_one_up_button"));
        get_custom_label_from_widget(&self.button_results_one_down).set_label(&fls!("upper_results_one_down_button"));
        self.menu_button_select_popup.set_label(&fls!("upper_select_popup_button"));
        self.file_chooser_dialog_add_files.set_title(&fls!("dialog_name_files_to_include"));
        self.file_chooser_dialog_add_folders.set_title(&fls!("dialog_name_folders_to_include"));
    }
    pub fn update_dark_theme(&self, is_dark_theme: bool) {
        if is_dark_theme {
            set_icon_of_button(&self.button_setting, SZY_ICON_SETTINGS_DARK);
            set_icon_of_button(&self.button_results_one_up, SZY_ICON_UP_DARK);
            set_icon_of_button(&self.button_results_one_down, SZY_ICON_DOWN_DARK);
        } else {
            set_icon_of_button(&self.button_setting, SZY_ICON_SETTINGS);
            set_icon_of_button(&self.button_results_one_up, SZY_ICON_UP);
            set_icon_of_button(&self.button_results_one_down, SZY_ICON_DOWN);
        }
    }
}
