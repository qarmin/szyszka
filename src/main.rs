// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::needless_late_init)]

mod class_dialog_rule_add_number;
mod class_dialog_rule_add_text;
mod class_dialog_rule_custom;
mod class_dialog_rule_normalize;
mod class_dialog_rule_purge;
mod class_dialog_rule_replace;
mod class_dialog_rule_size_letters;
mod class_dialog_rule_trim;
mod class_dialog_rules;
mod create_tree_view;
mod example_fields;
mod gui_connection;
mod gui_data;
mod gui_data_results;
mod gui_data_rules_bottom_panel;
mod gui_data_settings;
mod gui_data_upper_buttons;
mod gui_popover_select;
mod help_function;
mod initialize_gui;
mod language_functions;
mod localizer;
mod notebook_enum;
mod rule;
mod update_records;

use crate::example_fields::connect_update_examples;
use crate::gui_data::GuiData;
use crate::initialize_gui::*;
use gio::ApplicationFlags;
use glib::signal::Inhibit;
use gtk4::prelude::*;
use gtk4::Application;
use gui_connection::connect_add_files_button::*;
use gui_connection::connect_add_folders_button::*;
use gui_connection::connect_button_settings::*;
use gui_connection::connect_button_update_names::*;
use gui_connection::connect_change_language::*;
use gui_connection::connect_remove_files_button::*;
use gui_connection::connect_results_move::*;
use gui_connection::connect_rule_add::*;
use gui_connection::connect_rule_buttons_modify_rules::*;
use gui_connection::connect_rule_window_add_number_click::*;
use gui_connection::connect_rule_window_add_text_click::*;
use gui_connection::connect_rule_window_close::*;
use gui_connection::connect_rule_window_custom_click::*;
use gui_connection::connect_rule_window_normalize_click::*;
use gui_connection::connect_rule_window_purge_click::*;
use gui_connection::connect_rule_window_replace_click::*;
use gui_connection::connect_rule_window_size_letters_click::*;
use gui_connection::connect_rule_window_trim_click::*;
use gui_connection::connect_select_records::*;
use gui_connection::connect_start_renaming::*;
use std::env;
use std::ffi::OsString;

fn main() {
    let application = Application::new(None, ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE);
    application.connect_command_line(move |app, cmdline| {
        build_ui(app, cmdline.arguments());
        0
    });
    application.run_with_args(&env::args().collect::<Vec<_>>());
}

fn build_ui(application: &Application, _arguments: Vec<OsString>) {
    let gui_data: GuiData = GuiData::new_with_application(application);

    initialize_gui(&gui_data);

    connect_change_language(&gui_data);

    // Connect upper buttons
    connect_add_files_button(&gui_data);
    connect_add_folders_button(&gui_data);
    connect_remove_files_button(&gui_data);
    connect_button_settings(&gui_data);

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
    connect_rule_window_normalize_click(&gui_data);

    // Connect update examples in Rule Dialog
    connect_update_examples(&gui_data);

    // Connect rule buttons in main window
    connect_rule_modify_add(&gui_data);
    connect_rule_modify_edit(&gui_data);
    connect_rule_modify_remove(&gui_data);
    connect_rule_modify_one_up(&gui_data);
    connect_rule_modify_one_down(&gui_data);

    // Renaming
    connect_start_renaming(&gui_data);

    // Update Names
    connect_button_update_names(&gui_data);

    // Select
    connect_select_records(&gui_data);
    connect_select_all(&gui_data);
    connect_select_reverse(&gui_data);
    connect_select_custom(&gui_data);
    connect_unselect_custom(&gui_data);
    connect_select_changed(&gui_data);
    connect_unselect_changed(&gui_data);

    // Moving results
    connect_results_modify_one_up(&gui_data);
    connect_results_modify_one_down(&gui_data);

    let window_main = gui_data.window_main;
    window_main.connect_close_request(move |_| Inhibit(false));
}
