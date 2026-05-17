use crate::connect::sync::sync_files;
use crate::files::{regex_check, CHARACTER};
use crate::slint_gen::{MainWindow, SelectMode};
use crate::state::SharedState;

pub fn apply_select(ui: &MainWindow, state: &SharedState, mode: SelectMode) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);
        match mode {
            SelectMode::SelectAll => {
                for s in state_mut.file_selected.iter_mut() {
                    *s = true;
                }
            }
            SelectMode::Reverse => {
                for s in state_mut.file_selected.iter_mut() {
                    *s = !*s;
                }
            }
            SelectMode::SelectChanged => {
                let changed: Vec<usize> = state_mut
                    .files
                    .iter()
                    .enumerate()
                    .filter_map(|(i, f)| if f.future_name != f.name { Some(i) } else { None })
                    .collect();
                for idx in changed {
                    if let Some(s) = state_mut.file_selected.get_mut(idx) {
                        *s = true;
                    }
                }
            }
            SelectMode::UnselectChanged => {
                let changed: Vec<usize> = state_mut
                    .files
                    .iter()
                    .enumerate()
                    .filter_map(|(i, f)| if f.future_name != f.name { Some(i) } else { None })
                    .collect();
                for idx in changed {
                    if let Some(s) = state_mut.file_selected.get_mut(idx) {
                        *s = false;
                    }
                }
            }
            SelectMode::SelectCustom | SelectMode::UnselectCustom => {
                // handled by apply_select_custom
            }
        }
    }
    sync_files(ui, state);
}

pub fn apply_select_custom(ui: &MainWindow, state: &SharedState, pattern: &str, include_dirs: bool, mode_index: i32, select: bool) {
    {
        let mut state_mut = state.borrow_mut();
        let snapshot: Vec<(String, String, String, String, bool)> = state_mut
            .files
            .iter()
            .map(|f| (f.path.clone(), f.name.clone(), f.future_name.clone(), f.path.clone(), f.is_dir))
            .collect();

        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);

        for (idx, (path, current_name, future_name, _, is_dir)) in snapshot.iter().enumerate() {
            if *is_dir && !include_dirs {
                continue;
            }
            let target = match mode_index {
                0 => format!("{path}{CHARACTER}{current_name}"),
                1 => format!("{path}{CHARACTER}{future_name}"),
                2 => path.clone(),
                _ => path.clone(),
            };
            if regex_check(pattern, &target) {
                if let Some(s) = state_mut.file_selected.get_mut(idx) {
                    *s = select;
                }
            }
        }
    }
    sync_files(ui, state);
}
