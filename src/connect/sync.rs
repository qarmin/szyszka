use chrono::DateTime;
use humansize::{format_size, BINARY};
use slint::{ComponentHandle, ModelRc, VecModel};

use crate::slint_gen::{DirFileTypeUi, FileRow, GuiState, MainWindow, RuleRow};
use crate::state::SharedState;

pub fn sync_files(ui: &MainWindow, state: &SharedState) {
    let state_ref = state.borrow();
    let rows: Vec<FileRow> = state_ref
        .files
        .iter()
        .enumerate()
        .map(|(idx, item)| FileRow {
            selected: state_ref.file_selected.get(idx).copied().unwrap_or(false),
            dir_type: if item.is_dir { DirFileTypeUi::DirectoryT } else { DirFileTypeUi::FileT },
            current_name: item.name.clone().into(),
            future_name: item.future_name.clone().into(),
            path: item.path.clone().into(),
            size_text: format_size(item.size, BINARY).into(),
            date_text: item.date.clone().into(),
        })
        .collect();

    ui.set_files(ModelRc::new(VecModel::from(rows)));
    let g = ui.global::<GuiState>();
    g.set_file_count(state_ref.files.len() as i32);
}

pub fn sync_rules(ui: &MainWindow, state: &SharedState) {
    let state_ref = state.borrow();
    let rows: Vec<RuleRow> = state_ref
        .rules
        .rules
        .iter()
        .enumerate()
        .map(|(idx, rule)| RuleRow {
            selected: state_ref.rule_selected.get(idx).copied().unwrap_or(false),
            rule_type_text: crate::rule::rules::rule_type_to_string(rule.rule_type).into(),
            usage_text: crate::rule::rules::rule_place_to_string(rule.rule_place).into(),
            description: rule.rule_description.clone().into(),
        })
        .collect();

    let count = rows.len();
    ui.set_rules(ModelRc::new(VecModel::from(rows)));
    ui.global::<GuiState>().set_rule_count(count as i32);
}

pub fn sync_outdated(ui: &MainWindow, state: &SharedState) {
    let outdated = !state.borrow().rules.updated;
    ui.global::<GuiState>().set_results_outdated(outdated);
}

#[expect(dead_code)]
pub fn timestamp_to_date(ts: u64) -> String {
    DateTime::from_timestamp(ts as i64, 0)
        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}
