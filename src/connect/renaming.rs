use slint::{ComponentHandle, Timer, TimerMode};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use crate::connect::progress::{hide_overlay, show_overlay};
use crate::connect::sync::{sync_files, sync_outdated};
use crate::files::CHARACTER;
use crate::fls;
use crate::slint_gen::{GuiState, MainWindow, ProgressState};
use crate::state::SharedState;

struct RenameResult {
    properly_renamed: u32,
    ignored: u32,
    failed: Vec<(String, String, String)>,
}

pub fn start_renaming_request(ui: &MainWindow, state: &SharedState) {
    let gs = ui.global::<GuiState>();
    let state_ref = state.borrow();

    if state_ref.files.is_empty() {
        gs.set_message_dialog_title(fls!("renaming_missing_files").into());
        gs.set_message_dialog_text(fls!("renaming_require_missing_files").into());
        gs.set_message_dialog_open(true);
        return;
    }
    if state_ref.rules.rules.is_empty() {
        gs.set_message_dialog_title(fls!("renaming_missing_rules").into());
        gs.set_message_dialog_text(fls!("renaming_require_missing_rules").into());
        gs.set_message_dialog_open(true);
        return;
    }

    if !state_ref.rules.updated {
        gs.set_outdated_warning_open(true);
    } else {
        gs.set_confirm_dialog_open(true);
    }
}

pub fn perform_renaming(ui: &MainWindow, state: &SharedState) {
    let mut file_renames: Vec<(String, String)> = Vec::new();
    let mut folder_renames: BTreeMap<usize, Vec<(String, String)>> = BTreeMap::new();

    {
        let state_ref = state.borrow();
        for file in &state_ref.files {
            let old_name = format!("{}{}{}", file.path, CHARACTER, file.name);
            let new_name = format!("{}{}{}", file.path, CHARACTER, file.future_name);
            if file.is_dir {
                let depth = old_name.matches(CHARACTER).count();
                folder_renames.entry(depth).or_default().push((old_name, new_name));
            } else {
                file_renames.push((old_name, new_name));
            }
        }
    }

    let total = file_renames.len() + folder_renames.values().map(|v| v.len()).sum::<usize>();
    show_overlay(ui, "Renaming files…", "", false);
    let ps = ui.global::<ProgressState>();
    ps.set_total(total as i32);
    ps.set_current(0);

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_w = counter.clone();
    let (tx, rx) = mpsc::channel::<RenameResult>();

    let dest_exists_msg = fls!("renaming_destination_file_exists");

    std::thread::spawn(move || {
        let mut failed: Vec<(String, String, String)> = Vec::new();
        let mut properly_renamed: u32 = 0;
        let mut ignored: u32 = 0;

        for (old_name, new_name) in file_renames {
            rename_one(&old_name, &new_name, &mut ignored, &mut properly_renamed, &mut failed, &dest_exists_msg);
            counter_w.fetch_add(1, AtomicOrdering::Relaxed);
        }
        for (_depth, vec) in folder_renames.iter().rev() {
            for (old_name, new_name) in vec {
                rename_one(old_name, new_name, &mut ignored, &mut properly_renamed, &mut failed, &dest_exists_msg);
                counter_w.fetch_add(1, AtomicOrdering::Relaxed);
            }
        }

        let _ = tx.send(RenameResult {
            properly_renamed,
            ignored,
            failed,
        });
    });

    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let timer = Timer::default();
    let timer_holder: Rc<RefCell<Option<Timer>>> = Rc::new(RefCell::new(None));
    let th_c = timer_holder.clone();
    let counter_p = counter;

    timer.start(TimerMode::Repeated, Duration::from_millis(60), move || {
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };

        let ps = ui.global::<ProgressState>();
        let cur = counter_p.load(AtomicOrdering::Relaxed);
        ps.set_current(cur as i32);

        match rx.try_recv() {
            Ok(result) => {
                finalize_rename(&ui, &state_clone, result);
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

fn finalize_rename(ui: &MainWindow, state: &SharedState, result: RenameResult) {
    {
        let mut state_mut = state.borrow_mut();
        state_mut.files.clear();
        state_mut.file_selected.clear();
        state_mut.result_entries.files.clear();
    }

    let gs = ui.global::<GuiState>();
    gs.set_properly_renamed(result.properly_renamed as i32);
    gs.set_ignored_count(result.ignored as i32);

    if result.failed.is_empty() {
        gs.set_failed_text("".into());
    } else {
        let text_err = fls!("renaming_error");
        let mut text = String::new();
        for i in result.failed {
            text.push_str(&format!("{} -> {}, {text_err}: {}\n", i.0, i.1, i.2));
        }
        gs.set_failed_text(text.into());
    }

    hide_overlay(ui);
    gs.set_results_dialog_open(true);

    sync_files(ui, state);
    sync_outdated(ui, state);
}

fn rename_one(old_name: &str, new_name: &str, ignored: &mut u32, properly_renamed: &mut u32, failed_renames: &mut Vec<(String, String, String)>, dest_exists_msg: &str) {
    if new_name == old_name {
        *ignored += 1;
    } else if Path::new(new_name).exists() {
        failed_renames.push((old_name.to_string(), new_name.to_string(), dest_exists_msg.to_string()));
    } else if let Err(e) = fs::rename(old_name, new_name) {
        failed_renames.push((old_name.to_string(), new_name.to_string(), e.to_string()));
    } else {
        *properly_renamed += 1;
    }
}
