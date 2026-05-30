#![windows_subsystem = "windows"]

mod cli_arguments;
mod config;
mod connect;
mod files;
mod language;
mod localizer;
mod logger;
mod rule;
mod state;

mod slint_gen {
    slint::include_modules!();
}

use slint::ComponentHandle;

use crate::cli_arguments::{handle_help_version, parse_cli_paths};
use crate::config::{load_dark_theme_config_or_create, load_saved_language, save_dark_theme, save_language};
use crate::connect::files::{
    add_cli_paths, confirm_add_folders, move_selected_down, move_selected_up, pick_files_and_add, pick_folders_into_state, remove_selected, sort_files_by, SortKey,
};
use crate::connect::renaming::{copy_all_errors, perform_renaming, set_failed_page, start_renaming_request};
use crate::connect::rules_ops::{
    add_or_update_rule, close_editor, delete_custom_text, delete_rule_set, load_custom_text_into_editor, load_rule_set, move_rule_down, move_rule_up, open_editor,
    refresh_custom_texts, refresh_future_names, refresh_rule_sets, remove_rule, save_custom_text, save_rule_set, update_example,
};
use crate::connect::select::{apply_select, apply_select_custom, file_click_range, file_click_select, file_click_toggle, rule_click_range, rule_click_select, rule_click_toggle};
use crate::connect::sync::{sync_files, sync_outdated, sync_rules};
use crate::connect::translations::apply_translations;
use crate::language::apply_language;
use crate::slint_gen::{Callabler, GuiState, MainWindow, Settings, SortColumn};
use crate::state::new_shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args: Vec<String> = std::env::args().collect();
    handle_help_version(&cli_args);
    crate::logger::setup_logger();
    let cli_paths = parse_cli_paths(&cli_args);

    let saved_language = load_saved_language();
    apply_language(&saved_language);

    let ui = MainWindow::new()?;
    apply_translations(&ui);

    let state = new_shared();

    // Settings global
    let s = ui.global::<Settings>();
    s.set_dark_theme(load_dark_theme_config_or_create());
    s.set_selected_language(saved_language.into());

    refresh_rule_sets(&ui);
    refresh_custom_texts(&ui);

    let cb = ui.global::<Callabler>();

    // File operations
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_request_add_files(move || {
            if let Some(ui) = ui_weak.upgrade() {
                pick_files_and_add(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_request_pick_folders(move || {
            if let Some(ui) = ui_weak.upgrade() {
                if pick_folders_into_state(&ui, &state) {
                    ui.global::<GuiState>().set_add_folders_dialog_open(true);
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_confirm_add_folders(move |scan_inside, ignore_folders| {
            if let Some(ui) = ui_weak.upgrade() {
                confirm_add_folders(&ui, &state, scan_inside, ignore_folders);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_remove_selected_files(move || {
            if let Some(ui) = ui_weak.upgrade() {
                remove_selected(&ui, &state);
            }
        });
    }

    // Selection
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_select_action(move |mode| {
            if let Some(ui) = ui_weak.upgrade() {
                apply_select(&ui, &state, mode);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_select_custom_apply(move |pattern, include_dirs, mode_index, select_else| {
            if let Some(ui) = ui_weak.upgrade() {
                apply_select_custom(&ui, &state, pattern.as_str(), include_dirs, mode_index, select_else);
            }
        });
    }

    // Results ordering
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_results_move_up(move || {
            if let Some(ui) = ui_weak.upgrade() {
                move_selected_up(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_results_move_down(move || {
            if let Some(ui) = ui_weak.upgrade() {
                move_selected_down(&ui, &state);
            }
        });
    }

    // Update names (preview)
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_update_names(move || {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_future_names(&ui, &state);
                state.borrow_mut().rules.updated = true;
                sync_outdated(&ui, &state);
            }
        });
    }

    // Start renaming
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_start_renaming(move || {
            if let Some(ui) = ui_weak.upgrade() {
                start_renaming_request(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_confirm_renaming(move || {
            if let Some(ui) = ui_weak.upgrade() {
                perform_renaming(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_results_set_page(move |page| {
            if let Some(ui) = ui_weak.upgrade() {
                set_failed_page(&ui, &state, page);
            }
        });
    }
    {
        let state = state.clone();
        cb.on_results_copy_errors(move || {
            copy_all_errors(&state);
        });
    }

    // Rule editor
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_open_rule_editor(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let resolved = if idx == 0 {
                    let s = state.borrow();
                    s.rule_selected.iter().position(|x| *x).map_or(-1, |i| i as i32)
                } else {
                    idx
                };
                open_editor(&ui, &state, resolved);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_close_rule_editor(move || {
            if let Some(ui) = ui_weak.upgrade() {
                close_editor(&ui);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_editor_add(move || {
            if let Some(ui) = ui_weak.upgrade() {
                add_or_update_rule(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_editor_update_example(move || {
            if let Some(ui) = ui_weak.upgrade() {
                update_example(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_remove(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                remove_rule(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_move_up(move || {
            if let Some(ui) = ui_weak.upgrade() {
                move_rule_up(&ui, &state);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_move_down(move || {
            if let Some(ui) = ui_weak.upgrade() {
                move_rule_down(&ui, &state);
            }
        });
    }

    // Rule sets
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_set_save(move |name| {
            if let Some(ui) = ui_weak.upgrade() {
                save_rule_set(&ui, &state, name.as_str());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_set_load(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                load_rule_set(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_rule_set_delete(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                delete_rule_set(&ui, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_request_load_rule_sets(move || {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_rule_sets(&ui);
            }
        });
    }

    // Custom rule history
    {
        let ui_weak = ui.as_weak();
        cb.on_custom_rule_save(move |text| {
            if let Some(ui) = ui_weak.upgrade() {
                save_custom_text(&ui, text.as_str());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_custom_rule_load(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                load_custom_text_into_editor(&ui, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_custom_rule_delete(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                delete_custom_text(&ui, idx);
            }
        });
    }

    // Settings
    {
        cb.on_set_dark_theme(move |is_dark| {
            save_dark_theme(is_dark);
        });
    }
    {
        let ui_weak = ui.as_weak();
        cb.on_set_language(move |lang| {
            save_language(lang.as_str());
            apply_language(lang.as_str());
            if let Some(ui) = ui_weak.upgrade() {
                apply_translations(&ui);
            }
        });
    }
    {
        cb.on_open_rules_file(|| {
            if let Some(p) = crate::config::get_rules_config_file() {
                crate::config::create_rules_file_if_needed();
                let _ = open::that(p);
            }
        });
    }
    {
        cb.on_open_custom_texts_file(|| {
            if let Some(p) = crate::config::get_custom_text_config_file() {
                crate::config::create_custom_text_file_if_needed();
                let _ = open::that(p);
            }
        });
    }
    {
        cb.on_open_config_dir(|| {
            if let Some(p) = crate::config::get_config_path() {
                let _ = std::fs::create_dir_all(&p);
                let _ = open::that(p);
            }
        });
    }
    {
        cb.on_open_log_folder(|| {
            if let Some(p) = crate::logger::get_cache_path() {
                let _ = std::fs::create_dir_all(&p);
                let _ = open::that(p);
            }
        });
    }

    // Selection toggles for individual rows
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_set_rule_selected(move |idx, sel| {
            if let Some(ui) = ui_weak.upgrade() {
                {
                    let mut state_mut = state.borrow_mut();
                    let len = state_mut.rules.rules.len();
                    state_mut.rule_selected.resize(len, false);
                    if let Some(s) = state_mut.rule_selected.get_mut(idx as usize) {
                        *s = sel;
                    }
                }
                sync_rules(&ui, &state);
            }
        });
    }

    // Row-click selection (krokiet-style multi-select)
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_file_row_click_select(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                file_click_select(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_file_row_click_toggle(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                file_click_toggle(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_file_row_click_range(move |anchor, idx| {
            if let Some(ui) = ui_weak.upgrade() {
                file_click_range(&ui, &state, anchor, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_row_click_select(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                rule_click_select(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_row_click_toggle(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                rule_click_toggle(&ui, &state, idx);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_rule_row_click_range(move |anchor, idx| {
            if let Some(ui) = ui_weak.upgrade() {
                rule_click_range(&ui, &state, anchor, idx);
            }
        });
    }

    cb.on_filter_number(|input| input.chars().filter(char::is_ascii_digit).collect::<String>().into());

    {
        let state = state.clone();
        cb.on_open_file(move |idx| {
            let s = state.borrow();
            if let Some(item) = s.files.get(idx as usize) {
                let _ = open::that(&item.full_name);
            }
        });
    }
    {
        let state = state.clone();
        cb.on_open_file_folder(move |idx| {
            let s = state.borrow();
            if let Some(item) = s.files.get(idx as usize) {
                let _ = open::that(&item.path);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        cb.on_sort_files(move |column| {
            if let Some(ui) = ui_weak.upgrade() {
                let g = ui.global::<GuiState>();
                let prev_column = g.get_sort_column();
                let prev_desc = g.get_sort_descending();
                let (new_column, new_desc) = if prev_column == column {
                    if !prev_desc {
                        (column, true)
                    } else {
                        (SortColumn::None, false)
                    }
                } else {
                    (column, false)
                };
                g.set_sort_column(new_column);
                g.set_sort_descending(new_desc);
                let key = match new_column {
                    SortColumn::None => SortKey::None,
                    SortColumn::TypeC => SortKey::Type,
                    SortColumn::Current => SortKey::Current,
                    SortColumn::Future => SortKey::Future,
                    SortColumn::Path => SortKey::Path,
                };
                sort_files_by(&ui, &state, key, new_desc);
            }
        });
    }

    sync_files(&ui, &state);
    sync_rules(&ui, &state);
    sync_outdated(&ui, &state);

    add_cli_paths(&ui, &state, cli_paths);

    ui.run()?;
    Ok(())
}
