use crate::class_gui_data::GuiData;
use crate::help_function::populate_rules_tree_view;
use crate::rules::{RulePlace, RuleType};
use gtk::prelude::*;
use gtk::{ButtonExt, WidgetExt};
use std::ops::DerefMut;

pub fn connect_rule_ok(gui_data: &GuiData) {
    let button_dialog_ok = gui_data.dialog_rules.button_dialog_ok.clone();
    let dialog_with_rules = gui_data.dialog_rules.dialog_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let notebook_choose_rule = gui_data.dialog_rules.notebook_choose_rule.clone();
    let rules = gui_data.rules.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();

    let radio_button_letters_type_uppercase = gui_data.dialog_rules.radio_button_letters_type_uppercase.clone();
    let radio_button_letters_type_lowercase = gui_data.dialog_rules.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_usage_name = gui_data.dialog_rules.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = gui_data.dialog_rules.radio_button_letters_usage_extension.clone();
    let radio_button_letters_usage_both = gui_data.dialog_rules.radio_button_letters_usage_both.clone();

    button_dialog_ok.connect_clicked(move |_e| {
        dialog_with_rules.hide();
        window_main.set_sensitive(true);
        let mut rule = rules.borrow_mut();
        let rule = rule.deref_mut();

        let rule_type: RuleType;
        let rule_place: RulePlace;

        match notebook_choose_rule.get_current_page().unwrap() {
            0 => {
                if radio_button_letters_type_uppercase.get_active() {
                    rule_type = RuleType::UpperCase;
                } else if radio_button_letters_type_lowercase.get_active() {
                    rule_type = RuleType::LowerCase;
                } else {
                    panic!("Button not available");
                }
                if radio_button_letters_usage_extension.get_active() {
                    rule_place = RulePlace::Extension;
                } else if radio_button_letters_usage_both.get_active() {
                    rule_place = RulePlace::ExtensionAndName;
                } else if radio_button_letters_usage_name.get_active() {
                    rule_place = RulePlace::Name;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }
            _ => {
                panic!("Invalid notebook name");
            }
        }
        rule.add_rule(rule_type, rule_place);

        // TODO this should add rule to rules
        // tree_view_window_rules.

        // Reset TreeView and populate it again
        populate_rules_tree_view(&tree_view_window_rules, &rule);
    });
}
