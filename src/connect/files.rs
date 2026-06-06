use slint::{ComponentHandle, Timer, TimerMode};
use std::path::PathBuf;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use crate::connect::progress::{hide_overlay, show_overlay};
use crate::connect::rules_ops::refresh_outdated_or_recompute;
use crate::connect::sync::sync_files;
use crate::files::{collect_files_async, enumerate_folder_contents, sort_files, ItemStruct, ScanProgress};
use crate::slint_gen::{MainWindow, ProgressState};
use crate::state::SharedState;

pub fn pick_files_and_add(ui: &MainWindow, state: &SharedState) {
    let files = rfd::FileDialog::new().set_title("Add files").pick_files();
    let Some(files) = files else { return };
    let sorted = sort_files(files);
    start_async_scan(ui, state, sorted, "Adding files…");
}

pub fn add_cli_paths(ui: &MainWindow, state: &SharedState, paths: crate::cli_arguments::CliPaths) {
    if paths.is_empty() {
        return;
    }

    let mut items: Vec<PathBuf> = Vec::new();
    items.extend(sort_files(paths.files));
    if !paths.folders_normal.is_empty() {
        items.extend(enumerate_folder_contents(paths.folders_normal, false, false));
    }
    if !paths.folders_recursive.is_empty() {
        items.extend(enumerate_folder_contents(paths.folders_recursive, true, false));
    }
    if !paths.folders_recursive_skip.is_empty() {
        items.extend(enumerate_folder_contents(paths.folders_recursive_skip, true, true));
    }

    if !items.is_empty() {
        start_async_scan(ui, state, items, "Reading file metadata…");
    }
}

pub fn pick_folders_into_state(ui: &MainWindow, state: &SharedState) -> bool {
    let folders = rfd::FileDialog::new().set_title("Add folders").pick_folders();
    let Some(folders) = folders else { return false };
    if folders.is_empty() {
        return false;
    }

    let display: Vec<slint::SharedString> = folders.iter().map(|p| p.display().to_string().into()).collect();
    ui.global::<crate::slint_gen::GuiState>()
        .set_add_folder_picked_paths(slint::ModelRc::new(slint::VecModel::from(display)));
    state.borrow_mut().pending_folders = folders;
    true
}

pub fn confirm_add_folders(ui: &MainWindow, state: &SharedState, scan_inside: bool, ignore_folders: bool) {
    let folders = std::mem::take(&mut state.borrow_mut().pending_folders);
    if folders.is_empty() {
        return;
    }

    show_overlay(ui, "Scanning folders…", "Enumerating contents", true);

    let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();
    std::thread::spawn(move || {
        let items = enumerate_folder_contents(folders, scan_inside, ignore_folders);
        let _ = tx.send(items);
    });

    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let timer = Timer::default();

    timer.start(TimerMode::Repeated, Duration::from_millis(80), move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        match rx.try_recv() {
            Ok(items) => {
                // start_async_scan installs its own timer in `active_timer`, replacing (and
                // dropping) this one, which stops the current callback.
                start_async_scan(&ui, &state_clone, items, "Reading file metadata…");
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                hide_overlay(&ui);
                state_clone.borrow_mut().active_timer = None;
            }
        }
    });
    // Keep the timer alive so it keeps firing; the callback stops it by clearing this slot.
    state.borrow_mut().active_timer = Some(timer);
}

fn start_async_scan(ui: &MainWindow, state: &SharedState, items: Vec<PathBuf>, message: &str) {
    if items.is_empty() {
        hide_overlay(ui);
        return;
    }

    show_overlay(ui, "Scanning…", message, false);
    let ps = ui.global::<ProgressState>();
    ps.set_total(items.len() as i32);
    ps.set_current(0);

    let progress = Arc::new(ScanProgress::default());
    let dedup = state.borrow().result_entries.files.clone();
    let progress_w = progress.clone();

    let (tx, rx) = mpsc::channel::<Vec<ItemStruct>>();
    std::thread::spawn(move || {
        let result = collect_files_async(items, &dedup, &progress_w);
        let _ = tx.send(result);
    });

    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let timer = Timer::default();

    timer.start(TimerMode::Repeated, Duration::from_millis(70), move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };

        let ps = ui.global::<ProgressState>();
        let total = progress.total.load(AtomicOrdering::Relaxed);
        let current = progress.current.load(AtomicOrdering::Relaxed);
        if total > 0 {
            ps.set_indeterminate(false);
            ps.set_current(current as i32);
            ps.set_total(total as i32);
        }

        match rx.try_recv() {
            Ok(result) => {
                {
                    let mut s = state_clone.borrow_mut();
                    for item in &result {
                        s.result_entries.files.insert(item.full_name.clone());
                    }
                    s.files.extend(result);
                    let n = s.files.len();
                    s.file_selected.resize(n, false);
                    s.rules.updated = false;
                }
                sync_files(&ui, &state_clone);
                refresh_outdated_or_recompute(&ui, &state_clone);
                hide_overlay(&ui);
                // Drop the timer (held in state) to stop this repeated callback.
                state_clone.borrow_mut().active_timer = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                hide_overlay(&ui);
                state_clone.borrow_mut().active_timer = None;
            }
        }
    });
    // Keep the timer alive so it keeps firing; the callback stops it by clearing this slot.
    state.borrow_mut().active_timer = Some(timer);
}

pub fn remove_selected(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let to_remove: Vec<usize> = state_mut
            .file_selected
            .iter()
            .enumerate()
            .filter_map(|(i, sel)| if *sel { Some(i) } else { None })
            .collect();
        for idx in to_remove.iter().rev() {
            if let Some(removed) = state_mut.files.get(*idx).cloned() {
                state_mut.result_entries.files.remove(&removed.full_name);
            }
            state_mut.files.remove(*idx);
            state_mut.file_selected.remove(*idx);
        }
        if !to_remove.is_empty() {
            state_mut.rules.updated = false;
        }
    }
    sync_files(ui, state);
    refresh_outdated_or_recompute(ui, state);
}

pub fn move_selected_up(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        for i in 1..len {
            if state_mut.file_selected.get(i).copied().unwrap_or(false) && !state_mut.file_selected.get(i - 1).copied().unwrap_or(false) {
                state_mut.files.swap(i, i - 1);
                state_mut.file_selected.swap(i, i - 1);
            }
        }
    }
    sync_files(ui, state);
}

pub fn move_selected_down(ui: &MainWindow, state: &SharedState) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        if len == 0 {
            return;
        }
        for i in (0..len - 1).rev() {
            if state_mut.file_selected.get(i).copied().unwrap_or(false) && !state_mut.file_selected.get(i + 1).copied().unwrap_or(false) {
                state_mut.files.swap(i, i + 1);
                state_mut.file_selected.swap(i, i + 1);
            }
        }
    }
    sync_files(ui, state);
}

#[derive(Copy, Clone, PartialEq)]
pub enum SortKey {
    None,
    Type,
    Current,
    Future,
    Path,
}

pub fn sort_files_by(ui: &MainWindow, state: &SharedState, key: SortKey, descending: bool) {
    {
        let mut state_mut = state.borrow_mut();
        let len = state_mut.files.len();
        state_mut.file_selected.resize(len, false);

        let mut indices: Vec<usize> = (0..len).collect();
        let files = &state_mut.files;
        indices.sort_by(|&a, &b| {
            let fa = &files[a];
            let fb = &files[b];
            let ord = match key {
                SortKey::None => fa.path.cmp(&fb.path).then_with(|| natord::compare(&fa.name, &fb.name)),
                SortKey::Type => (!fa.is_dir).cmp(&!fb.is_dir).then_with(|| natord::compare(&fa.name, &fb.name)),
                SortKey::Current => natord::compare(&fa.name, &fb.name),
                SortKey::Future => natord::compare(&fa.future_name, &fb.future_name),
                SortKey::Path => natord::compare(&fa.path, &fb.path).then_with(|| natord::compare(&fa.name, &fb.name)),
            };
            if descending {
                ord.reverse()
            } else {
                ord
            }
        });

        let files = std::mem::take(&mut state_mut.files);
        let selected = std::mem::take(&mut state_mut.file_selected);
        let mut new_files = Vec::with_capacity(len);
        let mut new_selected = Vec::with_capacity(len);
        for i in indices {
            new_files.push(files[i].clone());
            new_selected.push(selected.get(i).copied().unwrap_or(false));
        }
        state_mut.files = new_files;
        state_mut.file_selected = new_selected;
    }
    sync_files(ui, state);
}
