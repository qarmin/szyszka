use gtk4::prelude::*;

use crate::class_dialog_rules::GuiDialogRules;
use crate::gui_data::GuiData;
use crate::help_function::{read_rule_from_window, validate_name};
use crate::notebook_enum::EXAMPLE_NAME;
use crate::rule::rules::Rules;

pub fn connect_update_examples(gui_data: &GuiData) {
    let notebook_choose_rule = gui_data.window_rules.notebook_choose_rule.clone();

    let button_example_reset = gui_data.window_rules.button_example_reset.clone();

    let window_rules = gui_data.window_rules.clone();
    notebook_choose_rule.connect_switch_page(move |_e, _y, z| {
        update_examples(&window_rules, Some(z));
    });

    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    button_example_reset.connect_clicked(move |_e| {
        entry_example_before.set_text(EXAMPLE_NAME);
    });

    let window_rules = gui_data.window_rules.clone();
    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    entry_example_before.connect_changed(move |e| {
        let old_name = e.text().to_string();
        let validate_name = validate_name(&old_name);
        if validate_name != old_name {
            e.set_text(&validate_name);
        }
        update_examples(&window_rules, None);
    });
}

pub fn update_examples(window_rules: &GuiDialogRules, notebook_number: Option<u32>) {
    let text_to_change: String = window_rules.entry_example_before.text().to_string();
    let label_example_after = window_rules.label_example_after.clone();

    let single_rule = read_rule_from_window(window_rules, notebook_number);

    let mut all_rules = Rules::new();
    all_rules.rules.push(single_rule);

    let text = all_rules.apply_all_rules_to_item(text_to_change, 1, 1, (0, 0, 0, "Parent folder"));
    label_example_after.set_text(text.as_str());
}
