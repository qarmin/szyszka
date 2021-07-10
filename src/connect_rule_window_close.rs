use crate::class_gui_data::GuiData;
use gtk::prelude::WidgetExtManual;
use gtk::prelude::*;

pub fn connect_rule_window_close(gui_data: &GuiData) {
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    window_with_rules.hide_on_delete();
    window_with_rules.connect_delete_event(move |e, _y| {
        window_main.set_sensitive(true);
        e.hide();
        gtk::Inhibit(true)
    });
}
