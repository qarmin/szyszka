use crate::gui_data::GuiData;
use gtk4::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_window_close(gui_data: &GuiData) {
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    let rules = gui_data.rules.clone();

    window_with_rules.connect_close_request(move |e| {
        let mut rules = rules.borrow_mut();
        let rules = rules.deref_mut();

        rules.edit_mode = None; // Reset in case of cancelling editing of rules

        window_main.set_sensitive(true);
        e.hide();
        gtk4::Inhibit(true)
    });
}