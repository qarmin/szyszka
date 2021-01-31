use crate::class_gui_data::GuiData;
use crate::example_fields::update_examples;
use crate::help_function::validate_name;
use gtk::{EditableSignals, EntryExt};

pub fn connect_rule_window_custom_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let entry_custom_text_to_change = gui_data.window_rules.custom.entry_custom_text_to_change.clone();

    entry_custom_text_to_change.connect_changed(move |e| {
        e.set_text(validate_name(e.get_text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
}
