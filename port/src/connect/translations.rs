use slint::ComponentHandle;

use crate::fls;
use crate::slint_gen::{MainWindow, Translations};

pub fn apply_translations(ui: &MainWindow) {
    let t = ui.global::<Translations>();

    t.set_upper_start_renaming_button(fls!("upper_start_renaming_button").into());
    t.set_upper_add_files_button(fls!("upper_add_files_button").into());
    t.set_upper_add_folders_button(fls!("upper_add_folders_button").into());
    t.set_upper_remove_selection_button(fls!("upper_remove_selection_button").into());
    t.set_upper_update_names_button(fls!("upper_update_names_button").into());
    t.set_upper_results_one_up_button(fls!("upper_results_one_up_button").into());
    t.set_upper_results_one_down_button(fls!("upper_results_one_down_button").into());
    t.set_upper_select_popup_button(fls!("upper_select_popup_button").into());

    t.set_bottom_rule_add_button(fls!("bottom_rule_add_button").into());
    t.set_bottom_rule_edit_button(fls!("bottom_rule_edit_button").into());
    t.set_bottom_rule_remove_button(fls!("bottom_rule_remove_button").into());
    t.set_bottom_rule_one_up_button(fls!("bottom_rule_one_up_button").into());
    t.set_bottom_rule_one_down_button(fls!("bottom_rule_one_down_button").into());
    t.set_bottom_rule_save_rules_button(fls!("bottom_rule_save_rules_button").into());
    t.set_bottom_rule_load_rules_button(fls!("bottom_rule_load_rules_button").into());

    t.set_bottom_rule_label_rules(fls!("bottom_rule_label_rules").into());
    t.set_upper_files_folders_label(fls!("upper_files_folders_label").into());

    t.set_tree_view_upper_column_type(fls!("tree_view_upper_column_type").into());
    t.set_tree_view_upper_column_current_name(fls!("tree_view_upper_column_current_name").into());
    t.set_tree_view_upper_column_future_name(fls!("tree_view_upper_column_future_name").into());
    t.set_tree_view_upper_column_path(fls!("tree_view_upper_column_path").into());

    t.set_tree_view_bottom_tool_type(fls!("tree_view_bottom_tool_type").into());
    t.set_tree_view_bottom_usage_name(fls!("tree_view_bottom_usage_name").into());
    t.set_tree_view_bottom_description(fls!("tree_view_bottom_description").into());

    t.set_settings_language_label(fls!("settings_language_label").into());
    t.set_check_button_dark_theme(fls!("check_button_dark_theme").into());
    t.set_settings_open_rules(fls!("settings_open_rules").into());
    t.set_settings_open_cache_custom_texts(fls!("settings_open_cache_custom_texts").into());
    t.set_settings_open_config_dir(fls!("settings_open_config_dir").into());
    t.set_settings_open_log_folder(fls!("settings_open_log_folder").into());

    t.set_button_select_all(fls!("button_select_all").into());
    t.set_button_unselect_all(fls!("button_unselect_all").into());
    t.set_button_select_reverse(fls!("button_select_reverse").into());
    t.set_button_select_custom(fls!("button_select_custom").into());
    t.set_button_select_changed(fls!("button_select_changed").into());
    t.set_button_unselect_changed(fls!("button_unselect_changed").into());

    t.set_dialog_button_ok(fls!("dialog_button_ok").into());
    t.set_dialog_button_cancel(fls!("dialog_button_cancel").into());

    t.set_dialog_name_files_to_include(fls!("dialog_name_files_to_include").into());
    t.set_dialog_name_folders_to_include(fls!("dialog_name_folders_to_include").into());
    t.set_dialog_scan_inside(fls!("dialog_scan_inside").into());
    t.set_dialog_ignore_folders(fls!("dialog_ignore_folders").into());
    t.set_dialog_confirm_renaming(fls!("dialog_confirm_renaming").into());
    t.set_dialog_outdated_results(fls!("dialog_outdated_results").into());
    t.set_renaming_some_records_not_updated(fls!("renaming_some_records_not_updated").into());
    t.set_dialog_results_of_renaming(fls!("dialog_results_of_renaming").into());
    t.set_dialog_save_rule(fls!("dialog_save_rule").into());
    t.set_edit_names_choose_name(fls!("edit_names_choose_name").into());
    t.set_dialog_select_custom(fls!("dialog_select_custom").into());

    t.set_label_usage_type(fls!("label_usage_type").into());
    t.set_label_example(fls!("label_example").into());
    t.set_label_example_text_before(fls!("label_example_text_before").into());
    t.set_label_example_text_after(fls!("label_example_text_after").into());
    t.set_button_rule_window_add(fls!("button_rule_window_add").into());

    t.set_label_custom_instruction(fls!("label_custom_instruction").into());
    t.set_menu_button_load_custom_rule(fls!("menu_button_load_custom_rule").into());
    t.set_button_save_custom_rule(fls!("button_save_custom_rule").into());

    t.set_check_button_letters_type_uppercase(fls!("check_button_letters_type_uppercase").into());
    t.set_check_button_letters_type_lowercase(fls!("check_button_letters_type_lowercase").into());
    t.set_check_button_letters_usage_name(fls!("check_button_letters_usage_name").into());
    t.set_check_button_letters_usage_extension(fls!("check_button_letters_usage_extension").into());
    t.set_check_button_letters_usage_both(fls!("check_button_letters_usage_both").into());
    t.set_label_letters_tool_type(fls!("label_letters_tool_type").into());

    t.set_label_purge_tool_type(fls!("label_purge_tool_type").into());
    t.set_check_button_purge_name(fls!("check_button_purge_name").into());
    t.set_check_button_purge_extension(fls!("check_button_purge_extension").into());
    t.set_check_button_purge_both(fls!("check_button_purge_both").into());

    t.set_label_add_number_place(fls!("label_add_number_place").into());
    t.set_label_add_number_settings(fls!("label_add_number_settings").into());
    t.set_check_button_add_number_before_name(fls!("check_button_add_number_before_name").into());
    t.set_check_button_add_number_after_name(fls!("check_button_add_number_after_name").into());
    t.set_label_number_start_number(fls!("label_number_start_number").into());
    t.set_label_number_step(fls!("label_number_step").into());
    t.set_label_number_fill_zeros(fls!("label_number_fill_zeros").into());

    t.set_check_button_add_text_before_name(fls!("check_button_add_text_before_name").into());
    t.set_check_button_add_text_after_name(fls!("check_button_add_text_after_name").into());
    t.set_label_add_text(fls!("label_add_text").into());

    t.set_check_button_replace_name(fls!("check_button_replace_name").into());
    t.set_check_button_replace_extension(fls!("check_button_replace_extension").into());
    t.set_check_button_replace_both(fls!("check_button_replace_both").into());
    t.set_check_button_replace_case_sensitive(fls!("check_button_replace_case_sensitive").into());
    t.set_check_button_replace_case_insensitive(fls!("check_button_replace_case_insensitive").into());
    t.set_check_button_replace_regex(fls!("check_button_replace_regex").into());
    t.set_check_button_replace_replace_all(fls!("check_button_replace_replace_all").into());
    t.set_label_replace_text_to_find(fls!("label_replace_text_to_find").into());
    t.set_label_replace_text_to_replace(fls!("label_replace_text_to_replace").into());
    t.set_label_replace_invalid_regex(fls!("label_replace_invalid_regex").into());

    t.set_check_button_trim_name_start(fls!("check_button_trim_name_start").into());
    t.set_check_button_trim_name_end(fls!("check_button_trim_name_end").into());
    t.set_check_button_trim_extension_start(fls!("check_button_trim_extension_start").into());
    t.set_check_button_trim_extension_end(fls!("check_button_trim_extension_end").into());
    t.set_check_button_trim_case_sensitive(fls!("check_button_trim_case_sensitive").into());
    t.set_check_button_trim_case_insensitive(fls!("check_button_trim_case_insensitive").into());
    t.set_label_trim_trim_text(fls!("label_trim_trim_text").into());
    t.set_label_trim_case_sensitivity(fls!("label_trim_case_sensitivity").into());

    t.set_label_normalize_name(fls!("label_normalize_name").into());
    t.set_check_button_normalize_everything(fls!("check_button_normalize_everything").into());
    t.set_check_button_normalize_partial(fls!("check_button_normalize_partial").into());

    t.set_notebook_tab_custom(fls!("notebook_tab_custom").into());
    t.set_notebook_tab_case_size(fls!("notebook_tab_case_size").into());
    t.set_notebook_tab_purge(fls!("notebook_tab_purge").into());
    t.set_notebook_tab_add_number(fls!("notebook_tab_add_number").into());
    t.set_notebook_tab_add_text(fls!("notebook_tab_add_text").into());
    t.set_notebook_tab_replace(fls!("notebook_tab_replace").into());
    t.set_notebook_tab_trim(fls!("notebook_tab_trim").into());
    t.set_notebook_tab_normalize(fls!("notebook_tab_normalize").into());
}
