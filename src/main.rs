// Remove console window in Windows OS
#![windows_subsystem = "windows"]

mod class_dialog_rule_add_number;
mod class_dialog_rule_add_text;
mod class_dialog_rule_custom;
mod class_dialog_rule_purge;
mod class_dialog_rule_replace;
mod class_dialog_rule_size_letters;
mod class_dialog_rule_trim;
mod class_dialog_rules;
mod class_gui_data;
mod class_popover_select;
mod class_results;
mod class_rules_bottom_panel;
mod class_upper_buttons;
mod connect_add_files_button;
mod connect_add_folders_button;
mod connect_remove_files_button;
mod connect_rule_add;
mod connect_rule_buttons_modify_rules;
mod connect_rule_window_add_number_click;
mod connect_rule_window_add_text_click;
mod connect_rule_window_close;
mod connect_rule_window_custom_click;
mod connect_rule_window_purge_click;
mod connect_rule_window_replace_click;
mod connect_rule_window_size_letters_click;
mod connect_rule_window_trim_click;
mod connect_select_records;
mod connect_start_renaming;
mod create_tree_view;
mod example_fields;
mod file_entry;
mod help_function;
mod initialize_gui;
mod notebook_enum;
mod rule_add_number;
mod rule_add_text;
mod rule_change_size_letters;
mod rule_custom;
mod rule_purge;
mod rule_replace;
mod rule_trim;
mod rules;
mod update_records;

use crate::class_gui_data::GuiData;
use crate::connect_add_files_button::*;
use crate::connect_add_folders_button::*;
use crate::connect_remove_files_button::*;
use crate::connect_rule_add::*;
use crate::connect_rule_buttons_modify_rules::*;
use crate::connect_rule_window_add_number_click::*;
use crate::connect_rule_window_add_text_click::*;
use crate::connect_rule_window_close::*;
use crate::connect_rule_window_custom_click::*;
use crate::connect_rule_window_purge_click::*;
use crate::connect_rule_window_replace_click::*;
use crate::connect_rule_window_size_letters_click::*;
use crate::connect_rule_window_trim_click::*;
use crate::connect_select_records::*;
use crate::connect_start_renaming::*;
use crate::example_fields::connect_update_examples;
use crate::initialize_gui::*;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let mut gui_data: GuiData = GuiData::new();

    initialize_gui(&mut gui_data);

    // Connect upper buttons
    connect_add_files_button(&gui_data);
    connect_add_folders_button(&gui_data);
    connect_remove_files_button(&gui_data);

    // Connect buttons OK and Close in select dialog
    connect_rule_window_close(&gui_data);

    // Connect buttons about rules at the bottom
    connect_rule_add(&gui_data);

    // Connect buttons in dialog to reflect change to examples
    connect_rule_window_size_letters_click(&gui_data);
    connect_rule_window_purge_click(&gui_data);
    connect_rule_window_add_text_click(&gui_data);
    connect_rule_window_trim_click(&gui_data);
    connect_rule_window_custom_click(&gui_data);
    connect_rule_window_replace_click(&gui_data);
    connect_rule_window_add_number_click(&gui_data);

    // Connect update examples in Rule Dialog
    connect_update_examples(&gui_data);

    // Connect rule buttons in main window
    connect_rule_modify_add(&gui_data);
    connect_rule_modify_remove(&gui_data);
    connect_rule_modify_one_up(&gui_data);

    // Renaming
    connect_start_renaming(&gui_data);

    // Select
    connect_select_records(&gui_data);
    connect_select_all(&gui_data);
    connect_select_reverse(&gui_data);
    connect_select_custom(&gui_data);
    connect_unselect_custom(&gui_data);

    // Quit the program when X in main window was clicked
    gui_data.window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();
}
