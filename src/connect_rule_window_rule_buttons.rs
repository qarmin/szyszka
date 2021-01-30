use crate::class_gui_data::GuiData;
use crate::update_records::{update_records, UpdateMode};
use gtk::{ButtonExt, WidgetExt};

pub fn connect_rule_window_rule_buttons(gui_data: &GuiData) {
    let button_add_rule = gui_data.rules_bottom_panel.button_add_rule.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    button_add_rule.connect_clicked(move |_e| {
        // window_rules.set_position(WindowPosition::CenterAlways);
        window_with_rules.show();
        window_main.set_sensitive(false);

        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleAdded);
    });
}
