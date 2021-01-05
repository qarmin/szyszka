use crate::class_gui_data::GuiData;
use gtk::{ButtonExt, WidgetExt};

pub fn connect_rule_close(gui_data: &GuiData) {
    let button_dialog_close = gui_data.dialog_rules.button_dialog_close.clone();
    let dialog_with_rules = gui_data.dialog_rules.dialog_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    button_dialog_close.connect_clicked(move |_e| {
        dialog_with_rules.hide();
        window_main.set_sensitive(true);
    });
}
