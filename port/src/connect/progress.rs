use slint::ComponentHandle;

use crate::slint_gen::{MainWindow, ProgressState};

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
