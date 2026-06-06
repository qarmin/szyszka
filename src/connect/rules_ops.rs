use std::collections::HashMap;

use regex::Regex;
use slint::{ComponentHandle, ModelRc, VecModel};

use crate::config::{load_custom_rules, load_rules, save_custom_rules, save_rules_to_file};
use crate::connect::sync::{sync_outdated, sync_rules};
use crate::fls;
use crate::localizer::generate_translation_hashmap;
use crate::rule::rules::{MultipleRules, RuleData, RulePlace, RuleType, Rules, SingleRule};
use crate::slint_gen::{Callabler, EditorState, GuiState, MainWindow, NotebookTab, RulePlaceUi, RuleSetEntry};
use crate::state::SharedState;

pub fn open_editor(ui: &MainWindow, state: &SharedState, edit_index: i32) {
    let gs = ui.global::<GuiState>();

    if edit_index >= 0 {
        let state_ref = state.borrow();
        let idx = edit_index as usize;
        if let Some(rule) = state_ref.rules.rules.get(idx).cloned() {
            drop(state_ref);
            load_rule_into_editor(ui, &rule);
            state.borrow_mut().edit_index = Some(idx);
        }
    } else {
        state.borrow_mut().edit_index = None;
        reset_editor(ui);
    }
    update_example(ui, state);
    gs.set_rule_editor_open(true);
}

pub fn close_editor(ui: &MainWindow) {
    ui.global::<GuiState>().set_rule_editor_open(false);
}

pub fn add_or_update_rule(ui: &MainWindow, state: &SharedState) {
    let single_rule = read_rule_from_editor(ui);

    {
        let mut state_mut = state.borrow_mut();
        if let Some(edit_idx) = state_mut.edit_index.take() {
            if let Some(slot) = state_mut.rules.rules.get_mut(edit_idx) {
                *slot = single_rule;
            } else {
                state_mut.rules.add_single_rule(single_rule);
            }
        } else {
            state_mut.rules.add_single_rule(single_rule);
        }
        state_mut.rules.updated = false;
        let new_len = state_mut.rules.rules.len();
        state_mut.rule_selected.resize(new_len, false);
    }
    sync_rules(ui, state);
    refresh_outdated_or_recompute(ui, state);
    ui.global::<GuiState>().set_rule_editor_open(false);
}

pub fn remove_rule(ui: &MainWindow, state: &SharedState, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        if idx < 0 {
            let to_remove: Vec<usize> = state_mut
                .rule_selected
                .iter()
                .enumerate()
                .filter_map(|(i, sel)| if *sel { Some(i) } else { None })
                .collect();
            for i in to_remove.iter().rev() {
                if *i < state_mut.rules.rules.len() {
                    state_mut.rules.rules.remove(*i);
                }
                if *i < state_mut.rule_selected.len() {
                    state_mut.rule_selected.remove(*i);
                }
            }
        } else {
            let i = idx as usize;
            if i < state_mut.rules.rules.len() {
                state_mut.rules.rules.remove(i);
            }
            if i < state_mut.rule_selected.len() {
                state_mut.rule_selected.remove(i);
            }
        }
        state_mut.rules.updated = false;
    }
    sync_rules(ui, state);
    refresh_outdated_or_recompute(ui, state);
}

pub fn move_rule_up(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.rules.rules.len();
        for i in 1..len {
            if state_mut.rule_selected.get(i).copied().unwrap_or(false) && !state_mut.rule_selected.get(i - 1).copied().unwrap_or(false) {
                state_mut.rules.rules.swap(i, i - 1);
                state_mut.rule_selected.swap(i, i - 1);
                state_mut.rules.updated = false;
            }
        }
    }
    sync_rules(ui, state);
    refresh_outdated_or_recompute(ui, state);
}

pub fn move_rule_down(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.rules.rules.len();
        if len == 0 {
            return;
        }
        for i in (0..len - 1).rev() {
            if state_mut.rule_selected.get(i).copied().unwrap_or(false) && !state_mut.rule_selected.get(i + 1).copied().unwrap_or(false) {
                state_mut.rules.rules.swap(i, i + 1);
                state_mut.rule_selected.swap(i, i + 1);
                state_mut.rules.updated = false;
            }
        }
    }
    sync_rules(ui, state);
    refresh_outdated_or_recompute(ui, state);
}

fn format_captures(regex: &Regex, text: &str) -> String {
    match regex.captures(text) {
        Some(caps) => {
            let n = caps.len();
            let header = fls!("label_replace_captures_number", generate_translation_hashmap(vec![("capture_number", n.to_string())]));
            let mut groups = Vec::with_capacity(n);
            for (i, m) in caps.iter().enumerate() {
                let s = m.map_or("", |x| x.as_str());
                groups.push(format!("{i}: {s}"));
            }
            format!("{header} - {}", groups.join(", "))
        }
        None => fls!("label_replace_no_captures"),
    }
}

pub fn update_example(ui: &MainWindow, state: &SharedState) {
    let es = ui.global::<EditorState>();

    let single_rule = read_rule_from_editor(ui);

    let regex = if single_rule.rule_data.use_regex {
        match Regex::new(&single_rule.rule_data.text_to_find) {
            Ok(r) => {
                es.set_replace_invalid_regex(false);
                Some(r)
            }
            Err(_) => {
                es.set_replace_invalid_regex(true);
                es.set_replace_captures_text("".into());
                None
            }
        }
    } else {
        es.set_replace_invalid_regex(false);
        es.set_replace_captures_text("".into());
        None
    };

    if let Some(r) = regex.as_ref() {
        let before = es.get_example_before_text().to_string();
        es.set_replace_captures_text(format_captures(r, &before).into());
    }

    let mut all_rules = Rules::new();
    all_rules.rules.push(single_rule);

    let before = es.get_example_before_text().to_string();
    let text = all_rules.apply_all_rules_to_item(before, 1, 1, (0, 0, 0, "Parent folder"), &[regex]);
    es.set_example_after_text(text.into());

    refresh_future_names(ui, state);
}

/// Skip auto-recompute when files * rules would freeze the UI on large datasets.
/// Why: GTK had the same heuristic at src/update_records.rs:16. User can still
/// trigger a manual recompute via the Update Names button.
const RULES_UPDATE_LIMIT: usize = 20000;

pub fn refresh_outdated_or_recompute(ui: &MainWindow, state: &SharedState) {
    let (files_n, rules_n) = {
        let s = state.borrow();
        (s.files.len(), s.rules.rules.len())
    };
    if rules_n == 0 {
        // No rules → future_name == name (ItemStruct init), nothing to recompute, nothing outdated.
        let mut state_mut = state.borrow_mut();
        for file in &mut state_mut.files {
            if file.future_name != file.name {
                file.future_name = file.name.clone();
            }
        }
        state_mut.rules.updated = true;
        drop(state_mut);
        crate::connect::sync::sync_files(ui, state);
    } else if files_n * rules_n <= RULES_UPDATE_LIMIT {
        refresh_future_names(ui, state);
        state.borrow_mut().rules.updated = true;
    }
    sync_outdated(ui, state);
}

pub fn refresh_future_names(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let rules_clone = state_mut.rules.clone();
        let compiled_regexes: Vec<Option<Regex>> = rules_clone
            .rules
            .iter()
            .map(|r| if r.rule_data.use_regex { Regex::new(&r.rule_data.text_to_find).ok() } else { None })
            .collect();

        // Indices are 1-based and the per-folder counter resets for each distinct path, matching
        // the original GTK behaviour so $(N) (global), $(K) (per-folder) and AddNumber stay correct.
        let mut folder_counter: HashMap<String, u32> = HashMap::new();
        for (idx, file) in state_mut.files.iter_mut().enumerate() {
            let in_folder = folder_counter.entry(file.path.clone()).or_insert(0);
            *in_folder += 1;
            let future = rules_clone.apply_all_rules_to_item(
                file.name.clone(),
                (idx + 1) as u64,
                *in_folder,
                (file.modification_date, file.creation_date, file.size, &file.path),
                &compiled_regexes,
            );
            file.future_name = future;
        }
    }
    crate::connect::sync::sync_files(ui, state);
}

fn read_rule_from_editor(ui: &MainWindow) -> SingleRule {
    let es = ui.global::<EditorState>();
    let mut rule_data = RuleData::new();
    let (rule_type, rule_place, rule_description) = match es.get_current_tab() {
        NotebookTab::Custom => {
            rule_data.custom_text = es.get_custom_text().to_string();
            let desc = fls!(
                "rule_description_custom_rule",
                generate_translation_hashmap(vec![("custom_rule", rule_data.custom_text.clone())])
            );
            (RuleType::Custom, RulePlace::None, desc)
        }
        NotebookTab::CaseSize => {
            rule_data.to_lowercase = es.get_case_lowercase();
            let place = ui_place_to_rule_place(es.get_case_place());
            let desc = if rule_data.to_lowercase {
                format!("{} {}", fls!("rule_description_lowercase"), fls!("rule_description_text"))
            } else {
                format!("{} {}", fls!("rule_description_uppercase"), fls!("rule_description_text"))
            };
            (RuleType::CaseSize, place, desc)
        }
        NotebookTab::Purge => {
            let place = ui_place_to_rule_place(es.get_purge_place());
            (RuleType::Purge, place, String::new())
        }
        NotebookTab::AddNumber => {
            let place = ui_place_to_rule_place(es.get_add_number_place());
            rule_data.number_start = es.get_add_number_start().to_string().parse::<i64>().unwrap_or(0);
            rule_data.number_step = es.get_add_number_step().to_string().parse::<i64>().unwrap_or(1);
            rule_data.fill_with_zeros = es.get_add_number_zeros().to_string().parse::<i64>().unwrap_or(0);
            let zeros = if rule_data.fill_with_zeros > 0 {
                format!(
                    " {}",
                    fls!(
                        "rule_description_zeros",
                        generate_translation_hashmap(vec![("zeros", rule_data.fill_with_zeros.to_string())])
                    )
                )
            } else {
                String::new()
            };
            let desc = fls!(
                "rule_description_step",
                generate_translation_hashmap(vec![
                    ("step", rule_data.number_step.to_string()),
                    ("start", rule_data.number_start.to_string()),
                    ("zeros", zeros),
                ])
            );
            (RuleType::AddNumber, place, desc)
        }
        NotebookTab::AddText => {
            let place = ui_place_to_rule_place(es.get_add_text_place());
            rule_data.add_text_text = es.get_add_text_text().to_string();
            let desc = format!("{} {}", fls!("rule_description_added_text"), rule_data.add_text_text);
            (RuleType::AddText, place, desc)
        }
        NotebookTab::Replace => {
            rule_data.case_sensitive = es.get_replace_case_sensitive();
            rule_data.use_regex = es.get_replace_use_regex();
            rule_data.regex_replace_all = es.get_replace_all_occurrences();
            rule_data.text_to_find = es.get_replace_text_to_find().to_string();
            rule_data.text_to_replace = es.get_replace_text_to_replace().to_string();
            let place = if rule_data.use_regex {
                RulePlace::None
            } else {
                ui_place_to_rule_place(es.get_replace_place())
            };
            let additional_regex_text = if rule_data.use_regex { " regex" } else { "" };
            let desc = fls!(
                "rule_description_replace",
                generate_translation_hashmap(vec![
                    ("text_to_find", rule_data.text_to_find.clone()),
                    ("text_to_replace", rule_data.text_to_replace.clone()),
                    ("additional_regex_text", additional_regex_text.to_string()),
                ])
            );
            (RuleType::Replace, place, desc)
        }
        NotebookTab::Trim => {
            rule_data.case_sensitive = es.get_trim_case_sensitive();
            rule_data.trim_text = es.get_trim_text().to_string();
            let place = ui_place_to_rule_place(es.get_trim_place());
            let where_remove = match place {
                RulePlace::FromNameStart => fls!("rule_description_start"),
                RulePlace::FromNameEndReverse => fls!("rule_description_end_of_name"),
                RulePlace::FromExtensionStart => fls!("rule_description_extension"),
                RulePlace::FromExtensionEndReverse => fls!("rule_description_end_of_extension"),
                _ => String::new(),
            };
            let desc = fls!(
                "rule_description_trimming",
                generate_translation_hashmap(vec![("trim_text", rule_data.trim_text.clone()), ("where_remove", where_remove)])
            );
            (RuleType::Trim, place, desc)
        }
        NotebookTab::Normalize => {
            rule_data.full_normalize = es.get_normalize_full();
            let desc = if rule_data.full_normalize {
                fls!("rule_description_full_normalize")
            } else {
                fls!("rule_description_partial_normalize")
            };
            (RuleType::Normalize, RulePlace::ExtensionAndName, desc)
        }
    };

    SingleRule {
        rule_type,
        rule_place,
        rule_data,
        rule_description,
    }
}

fn ui_place_to_rule_place(p: RulePlaceUi) -> RulePlace {
    match p {
        RulePlaceUi::NoneP => RulePlace::None,
        RulePlaceUi::Extension => RulePlace::Extension,
        RulePlaceUi::NameP => RulePlace::Name,
        RulePlaceUi::ExtensionAndName => RulePlace::ExtensionAndName,
        RulePlaceUi::BeforeExtension => RulePlace::BeforeExtension,
        RulePlaceUi::AfterExtension => RulePlace::AfterExtension,
        RulePlaceUi::BeforeName => RulePlace::BeforeName,
        RulePlaceUi::AfterName => RulePlace::AfterName,
        RulePlaceUi::FromNameStart => RulePlace::FromNameStart,
        RulePlaceUi::FromNameEndReverse => RulePlace::FromNameEndReverse,
        RulePlaceUi::FromExtensionStart => RulePlace::FromExtensionStart,
        RulePlaceUi::FromExtensionEndReverse => RulePlace::FromExtensionEndReverse,
    }
}

fn rule_place_to_ui_place(p: RulePlace) -> RulePlaceUi {
    match p {
        RulePlace::None => RulePlaceUi::NoneP,
        RulePlace::Extension => RulePlaceUi::Extension,
        RulePlace::Name => RulePlaceUi::NameP,
        RulePlace::ExtensionAndName => RulePlaceUi::ExtensionAndName,
        RulePlace::BeforeExtension => RulePlaceUi::BeforeExtension,
        RulePlace::AfterExtension => RulePlaceUi::AfterExtension,
        RulePlace::BeforeName => RulePlaceUi::BeforeName,
        RulePlace::AfterName => RulePlaceUi::AfterName,
        RulePlace::FromNameStart => RulePlaceUi::FromNameStart,
        RulePlace::FromNameEndReverse => RulePlaceUi::FromNameEndReverse,
        RulePlace::FromExtensionStart => RulePlaceUi::FromExtensionStart,
        RulePlace::FromExtensionEndReverse => RulePlaceUi::FromExtensionEndReverse,
    }
}

fn load_rule_into_editor(ui: &MainWindow, rule: &SingleRule) {
    let es = ui.global::<EditorState>();
    let tab = match rule.rule_type {
        RuleType::Custom => NotebookTab::Custom,
        RuleType::CaseSize => NotebookTab::CaseSize,
        RuleType::Purge => NotebookTab::Purge,
        RuleType::AddNumber => NotebookTab::AddNumber,
        RuleType::AddText => NotebookTab::AddText,
        RuleType::Replace => NotebookTab::Replace,
        RuleType::Trim => NotebookTab::Trim,
        RuleType::Normalize => NotebookTab::Normalize,
    };
    es.set_current_tab(tab);

    match rule.rule_type {
        RuleType::Custom => {
            es.set_custom_text(rule.rule_data.custom_text.clone().into());
        }
        RuleType::CaseSize => {
            es.set_case_lowercase(rule.rule_data.to_lowercase);
            es.set_case_place(rule_place_to_ui_place(rule.rule_place));
        }
        RuleType::Purge => {
            es.set_purge_place(rule_place_to_ui_place(rule.rule_place));
        }
        RuleType::AddNumber => {
            es.set_add_number_place(rule_place_to_ui_place(rule.rule_place));
            es.set_add_number_start(rule.rule_data.number_start.to_string().into());
            es.set_add_number_step(rule.rule_data.number_step.to_string().into());
            es.set_add_number_zeros(rule.rule_data.fill_with_zeros.to_string().into());
        }
        RuleType::AddText => {
            es.set_add_text_place(rule_place_to_ui_place(rule.rule_place));
            es.set_add_text_text(rule.rule_data.add_text_text.clone().into());
        }
        RuleType::Replace => {
            es.set_replace_place(rule_place_to_ui_place(rule.rule_place));
            es.set_replace_case_sensitive(rule.rule_data.case_sensitive);
            es.set_replace_use_regex(rule.rule_data.use_regex);
            es.set_replace_all_occurrences(rule.rule_data.regex_replace_all);
            es.set_replace_text_to_find(rule.rule_data.text_to_find.clone().into());
            es.set_replace_text_to_replace(rule.rule_data.text_to_replace.clone().into());
        }
        RuleType::Trim => {
            es.set_trim_place(rule_place_to_ui_place(rule.rule_place));
            es.set_trim_case_sensitive(rule.rule_data.case_sensitive);
            es.set_trim_text(rule.rule_data.trim_text.clone().into());
        }
        RuleType::Normalize => {
            es.set_normalize_full(rule.rule_data.full_normalize);
        }
    }
}

fn reset_editor(ui: &MainWindow) {
    let es = ui.global::<EditorState>();
    es.set_current_tab(NotebookTab::Custom);
    es.set_custom_text("FILE_$(N).$(EXT)".into());
    es.set_case_lowercase(true);
    es.set_case_place(RulePlaceUi::NameP);
    es.set_purge_place(RulePlaceUi::NameP);
    es.set_add_number_place(RulePlaceUi::BeforeName);
    es.set_add_number_start("0".into());
    es.set_add_number_step("1".into());
    es.set_add_number_zeros("0".into());
    es.set_add_text_place(RulePlaceUi::BeforeName);
    es.set_add_text_text("".into());
    es.set_replace_place(RulePlaceUi::NameP);
    es.set_replace_case_sensitive(false);
    es.set_replace_use_regex(false);
    es.set_replace_all_occurrences(true);
    es.set_replace_text_to_find("".into());
    es.set_replace_text_to_replace("".into());
    es.set_trim_place(RulePlaceUi::FromNameStart);
    es.set_trim_case_sensitive(false);
    es.set_trim_text("".into());
    es.set_normalize_full(true);
    es.set_example_before_text("Gżegżółka.Txt".into());
}

pub fn save_rule_set(ui: &MainWindow, state: &SharedState, name: &str) {
    if name.trim().is_empty() {
        return;
    }
    let rules = state.borrow().rules.rules.clone();
    let new_entry = MultipleRules { name: name.to_string(), rules };
    let mut all = load_rules();
    if let Some(existing) = all.iter_mut().find(|m| m.name == name) {
        *existing = new_entry;
    } else {
        all.push(new_entry);
    }
    save_rules_to_file(&all);
    refresh_rule_sets(ui);
}

pub fn load_rule_set(ui: &MainWindow, state: &SharedState, index: i32) {
    if index < 0 {
        return;
    }
    let all = load_rules();
    if let Some(entry) = all.get(index as usize) {
        {
            let mut state_mut = state.borrow_mut();
            state_mut.rules.rules = entry.rules.clone();
            state_mut.rules.updated = false;
            let len = state_mut.rules.rules.len();
            state_mut.rule_selected.clear();
            state_mut.rule_selected.resize(len, false);
        }
        sync_rules(ui, state);
        refresh_outdated_or_recompute(ui, state);
    }
}

pub fn delete_rule_set(ui: &MainWindow, index: i32) {
    if index < 0 {
        return;
    }
    let mut all = load_rules();
    if (index as usize) < all.len() {
        all.remove(index as usize);
        save_rules_to_file(&all);
    }
    refresh_rule_sets(ui);
}

pub fn refresh_rule_sets(ui: &MainWindow) {
    let all = load_rules();
    let names: Vec<String> = all.iter().map(|m| m.name.clone()).collect();
    let names_text = if names.is_empty() {
        String::new()
    } else {
        fls!("edit_names_used_in_rules", generate_translation_hashmap(vec![("rules", names.join(", "))]))
    };
    ui.global::<GuiState>().set_existing_rule_set_names(names_text.into());

    let entries: Vec<RuleSetEntry> = all.into_iter().map(|m| RuleSetEntry { name: m.name.into() }).collect();
    ui.global::<Callabler>().set_saved_rule_sets(ModelRc::new(VecModel::from(entries)));
}

pub fn refresh_custom_texts(ui: &MainWindow) {
    let texts = load_custom_rules();
    let model: Vec<slint::SharedString> = texts.into_iter().map(Into::into).collect();
    ui.global::<Callabler>().set_saved_custom_rules(ModelRc::new(VecModel::from(model)));
}

pub fn save_custom_text(ui: &MainWindow, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    let mut current = load_custom_rules();
    if !current.iter().any(|t| t == text) {
        current.push(text.to_string());
        save_custom_rules(&current);
    }
    refresh_custom_texts(ui);
}

pub fn load_custom_text_into_editor(ui: &MainWindow, index: i32) {
    if index < 0 {
        return;
    }
    let all = load_custom_rules();
    if let Some(text) = all.get(index as usize) {
        ui.global::<EditorState>().set_custom_text(text.clone().into());
    }
}

pub fn delete_custom_text(ui: &MainWindow, index: i32) {
    if index < 0 {
        return;
    }
    let mut all = load_custom_rules();
    if (index as usize) < all.len() {
        all.remove(index as usize);
        save_custom_rules(&all);
    }
    refresh_custom_texts(ui);
}
