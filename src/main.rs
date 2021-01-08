mod class_dialog_rule_purge;
mod class_dialog_rule_size_letters;
mod class_dialog_rules;
mod class_gui_data;
mod class_results;
mod class_rules_bottom_panel;
mod class_status;
mod class_upper_buttons;
mod connect_add_files_button;
mod connect_dialog_purge_click;
mod connect_dialog_rule_buttons;
mod connect_dialog_size_letters_click;
mod connect_rule_close;
mod connect_rule_ok;
mod create_tree_view;
mod file_entry;
mod help_function;
mod initialize_gui;
mod rule_change_size_letters;
mod rule_purge;
mod rules;
mod update_records;

use crate::class_gui_data::GuiData;
use crate::connect_add_files_button::*;
use crate::connect_dialog_purge_click::*;
use crate::connect_dialog_rule_buttons::*;
use crate::connect_dialog_size_letters_click::*;
use crate::connect_rule_close::*;
use crate::connect_rule_ok::*;
use crate::initialize_gui::*;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let mut gui_data: GuiData = GuiData::new();

    initialize_gui(&mut gui_data);

    // Connect upper buttons
    connect_add_files_button(&gui_data);

    // Connect buttons OK and Close in select dialog
    connect_rule_close(&gui_data);
    connect_rule_ok(&gui_data);

    // Connect buttons in dialog to reflect change to examples
    connect_dialog_size_letters_click(&gui_data);
    connect_dialog_purge_click(&gui_data);

    // Connect rule buttons in main window
    connect_dialog_rule_buttons(&gui_data);

    // Quit the program when X in main window was clicked
    gui_data.window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();
}
