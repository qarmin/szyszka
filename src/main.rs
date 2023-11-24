// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::needless_late_init)]

use std::env;

use gio::ApplicationFlags;
use glib::Propagation;
use gtk4::prelude::*;
use gtk4::Application;

use crate::cli_arguments::{parse_cli_arguments, parse_cli_help_version_arguments};
use crate::config::load_dark_theme_config_or_create;
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

use crate::example_fields::connect_update_examples;
use crate::gui_connection::connect_settings::connect_settings_buttons;
use crate::gui_connection::connect_window_rules_open::connect_window_rules_open;
use crate::gui_data_things::gui_data::GuiData;
use crate::initialize_gui::*;

mod add_files_folders;
mod cli_arguments;
mod config;
mod create_tree_view;
mod example_fields;
mod gui_connection;
mod gui_data_things;
mod help_function;
mod initialize_gui;
mod language_functions;
mod localizer;
mod notebook_enum;
mod rule;
mod rule_read;
mod update_records;

fn main() {
    let application = Application::new(None::<String>, ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE);
    application.connect_command_line(move |app, cmdline| {
        build_ui(app, &cmdline.arguments().into_iter().map(|e| e.to_string_lossy().to_string()).collect::<Vec<_>>());
        0
    });
    application.run_with_args(&env::args().collect::<Vec<String>>());
}

fn build_ui(application: &Application, arguments: &[String]) {
    parse_cli_help_version_arguments(arguments);

    let gui_data: GuiData = GuiData::new_with_application(application);
    gui_data.update_dark_theme(load_dark_theme_config_or_create());

    initialize_gui(&gui_data);

    load_language(&gui_data);
    connect_change_language(&gui_data);

    // Connect upper buttons
    connect_add_files_button(&gui_data);
    connect_add_folders_button(&gui_data);
    connect_remove_files_button(&gui_data);
    connect_button_settings(&gui_data);

    // Connect buttons OK and Close in select dialog
    connect_rule_window_close(&gui_data);
    connect_window_rules_open(&gui_data);
    connect_settings_buttons(&gui_data);

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
    connect_rule_load(&gui_data);
    connect_rule_save(&gui_data);

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

    parse_cli_arguments(&gui_data, arguments);

    let gui_data_cloned = gui_data.clone();
    let window_main = gui_data.window_main;
    window_main.connect_close_request(move |_| {
        let gui_data_cloned = gui_data_cloned.clone();
        save_language(&gui_data_cloned);
        Propagation::Stop
    });
}
