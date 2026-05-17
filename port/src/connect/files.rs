use slint::{ComponentHandle, Timer, TimerMode};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use crate::connect::progress::{hide_overlay, show_overlay};
use crate::connect::sync::{sync_files, sync_outdated};
use crate::files::{collect_files_async, enumerate_folder_contents, sort_files, ItemStruct, ScanProgress};
use crate::slint_gen::{MainWindow, ProgressState};
use crate::state::SharedState;

pub fn pick_files_and_add(ui: &MainWindow, state: &SharedState) {
    let files = rfd::FileDialog::new().set_title("Add files").pick_files();
    let Some(files) = files else { return };
    let sorted = sort_files(files);
    start_async_scan(ui, state, sorted, "Adding files…");
}

pub fn pick_folders_and_add(ui: &MainWindow, state: &SharedState, scan_inside: bool, ignore_folders: bool) {
    let folders = rfd::FileDialog::new().set_title("Add folders").pick_folders();
    let Some(folders) = folders else { return };

    show_overlay(ui, "Scanning folders…", "Enumerating contents", true);

    let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();
    std::thread::spawn(move || {
        let items = enumerate_folder_contents(folders, scan_inside, ignore_folders);
        let _ = tx.send(items);
    });

    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let timer = Timer::default();
    let timer_holder: Rc<RefCell<Option<Timer>>> = Rc::new(RefCell::new(None));
    let th_c = timer_holder.clone();

    timer.start(TimerMode::Repeated, Duration::from_millis(80), move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };
        match rx.try_recv() {
            Ok(items) => {
                if let Some(t) = th_c.borrow().as_ref() {
                    t.stop();
                }
                start_async_scan(&ui, &state_clone, items, "Reading file metadata…");
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                hide_overlay(&ui);
                if let Some(t) = th_c.borrow().as_ref() {
                    t.stop();
                }
            }
        }
    });
    *timer_holder.borrow_mut() = Some(timer);
    state.borrow_mut().active_timer = timer_holder.borrow_mut().take();
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
        let result = collect_files_async(items, dedup, progress_w);
        let _ = tx.send(result);
    });

    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let timer = Timer::default();
    let timer_holder: Rc<RefCell<Option<Timer>>> = Rc::new(RefCell::new(None));
    let th_c = timer_holder.clone();

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
                sync_outdated(&ui, &state_clone);
                hide_overlay(&ui);
                if let Some(t) = th_c.borrow().as_ref() {
                    t.stop();
                }
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                hide_overlay(&ui);
                if let Some(t) = th_c.borrow().as_ref() {
                    t.stop();
                }
            }
        }
    });
    *timer_holder.borrow_mut() = Some(timer);
    state.borrow_mut().active_timer = timer_holder.borrow_mut().take();
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
    sync_outdated(ui, state);
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
