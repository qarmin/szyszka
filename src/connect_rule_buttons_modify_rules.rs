use crate::class_gui_data::GuiData;
use crate::help_function::{populate_rules_tree_view, remove_selected_rows};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_modify_add(gui_data: &GuiData) {
    let button_add_rule = gui_data.rules_bottom_panel.button_add_rule.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    button_add_rule.connect_clicked(move |_e| {
        // window_rules.set_position(WindowPosition::Center);
        window_with_rules.show();
        window_main.set_sensitive(false);
    });
}

pub fn connect_rule_modify_remove(gui_data: &GuiData) {
    let button_remove_rule = gui_data.rules_bottom_panel.button_remove_rule.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    // Multiselection Ready
    button_remove_rule.connect_clicked(move |_e| {
        let vec_rule_to_delete = remove_selected_rows(&tree_view_window_rules);

        {
            let mut rules = rules.borrow_mut();
            let rules = rules.deref_mut();

            for rule_to_delete in vec_rule_to_delete.iter().rev() {
                rules.remove_rule(*rule_to_delete);
            }
        }

        // Reset TreeView and populate it again
        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleRemoved);
        populate_rules_tree_view(&tree_view_window_rules, rules.clone());
    });
}

pub fn connect_rule_modify_one_up(gui_data: &GuiData) {
    let button_one_up = gui_data.rules_bottom_panel.button_one_up.clone();
    // let tree_view_results = gui_data.results.tree_view_results.clone();
    // let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    // let shared_result_entries = gui_data.shared_result_entries.clone();
    // let rules = gui_data.rules.clone();

    // TODO only works with single selection
    button_one_up.connect_clicked(move |_e| {
        // { // TODO Enable
        //     let selection = tree_view_window_rules.selection();
        //     if selection.get_selected_rows().0.is_empty() {
        //         return;
        //     }
        //
        //     let mut rules = rules.borrow_mut();
        //     let rules = rules.deref_mut();
        //
        //     let list_store = get_list_store_from_tree_view(&tree_view_window_rules);
        //
        //     let (selected_rows, _tree_model) = selection.get_selected_rows();
        //
        //     let mut vec_rule_to_move: Vec<_> = Vec::new();
        //     let mut current_iter: usize = 0;
        //
        //     let tree_iter = list_store.iter_first().unwrap();
        //
        //     // Get rules number to move
        //     for selected_tree_path in &selected_rows {
        //         loop {
        //             if list_store.get_path(&tree_iter).unwrap() == *selected_tree_path {
        //                 vec_rule_to_move.push(current_iter);
        //                 list_store.iter_next(&tree_iter);
        //                 current_iter += 1;
        //                 break;
        //             }
        //             list_store.iter_next(&tree_iter);
        //             current_iter += 1;
        //         }
        //     }
        //
        //     // move rules
        //     for rule_to_move in &vec_rule_to_move {
        //         if *rule_to_move != 0 {
        //             rules.rule_data.swap(*rule_to_move, rule_to_move - 1);
        //             rules.rule_place.swap(*rule_to_move, rule_to_move - 1);
        //             rules.rule_types.swap(*rule_to_move, rule_to_move - 1);
        //         }
        //     }
        //
        //     // Update rules in rule window
        //
        //     for selected_tree_path in selected_rows.iter().rev() {
        //         let current_iter = list_store.get_iter(&selected_tree_path).unwrap();
        //         let previous_iter = current_iter.clone();
        //
        //         if !list_store.iter_previous(&previous_iter) {
        //             continue; // First iter, there is no place to move it higher
        //         }
        //
        //         list_store.swap(&current_iter, &previous_iter);
        //     }
        //
        //     // Restore selection
        //
        //     // let mut current_rule = 0;
        //     // let iter = list_store.iter_first().unwrap();
        //     // selection.unselect_all();
        //     // selection.select_iter(&iter);
        //     // selection.select_iter(&iter);
        //     // selection.select_all();
        //     // loop {
        //     //     if current_rule == max(0, vec_rule_to_move[0]) {
        //     //         selection.select_iter(&iter);
        //     //         break;
        //     //     }
        //     //     list_store.iter_next(&iter);
        //     //     current_rule += 1;
        //     // }
        //     // for rule_to_move in vec_rule_to_move {
        //     //     loop {
        //     //         if rule_to_move == current_rule
        //     //     }
        //     // }
        // }
        //
        // // Reset TreeView and populate it again
        // update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleRemoved);
        // populate_rules_tree_view(&tree_view_window_rules, rules.clone());
    });
}
