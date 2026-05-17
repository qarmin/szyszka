use slint::{ComponentHandle, Timer, TimerMode};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::Arc;
use std::time::Duration;

use crate::files::ScanProgress;
use crate::slint_gen::{MainWindow, ProgressState};

pub type TimerHolder = Rc<RefCell<Option<Timer>>>;

pub fn show_overlay(ui: &MainWindow, title: &str, message: &str, indeterminate: bool) {
    let ps = ui.global::<ProgressState>();
    ps.set_visible(true);
    ps.set_title(title.into());
    ps.set_message(message.into());
    ps.set_indeterminate(indeterminate);
    ps.set_current(0);
    ps.set_total(0);
}

pub fn hide_overlay(ui: &MainWindow) {
    ui.global::<ProgressState>().set_visible(false);
}

pub fn poll_scan<T: 'static, F>(ui: &MainWindow, progress: Arc<ScanProgress>, rx: Receiver<T>, on_done: F) -> TimerHolder
where
    F: Fn(&MainWindow, T) + 'static,
{
    let timer_holder: TimerHolder = Rc::new(RefCell::new(None));
    let timer = Timer::default();

    let ui_weak = ui.as_weak();
    let timer_holder_c = timer_holder.clone();

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
                on_done(&ui, result);
                if let Some(t) = timer_holder_c.borrow().as_ref() {
                    t.stop();
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                ps.set_visible(false);
                if let Some(t) = timer_holder_c.borrow().as_ref() {
                    t.stop();
                }
            }
        }
    });

    *timer_holder.borrow_mut() = Some(timer);
    timer_holder
}
