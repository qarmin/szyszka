mod class_gui_data;
mod class_results;
mod class_rules;
mod class_status;
mod class_upper_buttons;
mod connect_add_files_button;
mod connect_tree_view;
mod file_entry;
mod help_function;
mod initialize_gui;
mod recalculate_results;

use crate::class_gui_data::GuiData;
use crate::connect_add_files_button::*;
use crate::initialize_gui::*;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let mut gui_data: GuiData = GuiData::new();

    initialize_gui(&mut gui_data);
    connect_add_files_button(&gui_data);

    // Quit the program when X in main window was clicked
    gui_data.window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();
}
