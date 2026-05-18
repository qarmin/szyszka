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
                for s in &mut state_mut.file_selected {
                    *s = true;
                }
            }
            SelectMode::UnselectAll => {
                for s in &mut state_mut.file_selected {
                    *s = false;
                }
            }
            SelectMode::Reverse => {
                for s in &mut state_mut.file_selected {
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

pub fn file_click_select(ui: &MainWindow, state: &SharedState, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);
        for s in &mut state_mut.file_selected {
            *s = false;
        }
        if let Some(s) = state_mut.file_selected.get_mut(idx as usize) {
            *s = true;
        }
    }
    sync_files(ui, state);
}

pub fn file_click_toggle(ui: &MainWindow, state: &SharedState, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);
        if let Some(s) = state_mut.file_selected.get_mut(idx as usize) {
            *s = !*s;
        }
    }
    sync_files(ui, state);
}

pub fn file_click_range(ui: &MainWindow, state: &SharedState, anchor: i32, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);
        for s in &mut state_mut.file_selected {
            *s = false;
        }
        let (lo, hi) = if anchor <= idx { (anchor, idx) } else { (idx, anchor) };
        let lo = lo.max(0) as usize;
        let hi = (hi as usize).min(len.saturating_sub(1));
        for s in state_mut.file_selected.iter_mut().take(hi + 1).skip(lo) {
            *s = true;
        }
    }
    sync_files(ui, state);
}

pub fn rule_click_select(ui: &MainWindow, state: &SharedState, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.rules.rules.len();
        state_mut.rule_selected.resize(len, false);
        for s in &mut state_mut.rule_selected {
            *s = false;
        }
        if let Some(s) = state_mut.rule_selected.get_mut(idx as usize) {
            *s = true;
        }
    }
    crate::connect::sync::sync_rules(ui, state);
}

pub fn rule_click_toggle(ui: &MainWindow, state: &SharedState, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.rules.rules.len();
        state_mut.rule_selected.resize(len, false);
        if let Some(s) = state_mut.rule_selected.get_mut(idx as usize) {
            *s = !*s;
        }
    }
    crate::connect::sync::sync_rules(ui, state);
}

pub fn rule_click_range(ui: &MainWindow, state: &SharedState, anchor: i32, idx: i32) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.rules.rules.len();
        state_mut.rule_selected.resize(len, false);
        for s in &mut state_mut.rule_selected {
            *s = false;
        }
        let (lo, hi) = if anchor <= idx { (anchor, idx) } else { (idx, anchor) };
        let lo = lo.max(0) as usize;
        let hi = (hi as usize).min(len.saturating_sub(1));
        for s in state_mut.rule_selected.iter_mut().take(hi + 1).skip(lo) {
            *s = true;
        }
    }
    crate::connect::sync::sync_rules(ui, state);
}

// Mode indexes must match the combobox order in main.slint:
//   0 = Path, 1 = Current Name, 2 = Future Name,
//   3 = Path + Current Name, 4 = Path + Future Name, 5 = Directory / File
pub fn apply_select_custom(ui: &MainWindow, state: &SharedState, pattern: &str, include_dirs: bool, mode_index: i32, select: bool) {
    {
        let mut state_mut = state.borrow_mut();
        let snapshot: Vec<(String, String, String, bool)> = state_mut.files.iter().map(|f| (f.path.clone(), f.name.clone(), f.future_name.clone(), f.is_dir)).collect();

        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);

        for (idx, (path, current_name, future_name, is_dir)) in snapshot.iter().enumerate() {
            let matched = match mode_index {
                5 => {
                    // IsDir: pattern ignored; include_dirs decides whether to select dirs or files
                    *is_dir == include_dirs
                }
                _ => {
                    if *is_dir && !include_dirs {
                        false
                    } else {
                        let target = match mode_index {
                            1 => current_name.clone(),
                            2 => future_name.clone(),
                            3 => format!("{path}{CHARACTER}{current_name}"),
                            4 => format!("{path}{CHARACTER}{future_name}"),
                            _ => path.clone(),
                        };
                        regex_check(pattern, &target)
                    }
                }
            };
            if matched {
                if let Some(s) = state_mut.file_selected.get_mut(idx) {
                    *s = select;
                }
            }
        }
    }
    sync_files(ui, state);
}
