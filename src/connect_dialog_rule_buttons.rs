use crate::class_gui_data::GuiData;
use gtk::{ButtonExt, DialogExt, WidgetExt};

pub fn connect_dialog_rule_buttons(gui_data: &GuiData) {
    let button_add_rule = gui_data.rules_bottom_panel.button_add_rule.clone();
    let dialog_rules = gui_data.dialog_rules.dialog_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    button_add_rule.connect_clicked(move |_e| {
        // dialog_rules.set_position(WindowPosition::CenterAlways);
        dialog_rules.show();
        window_main.set_sensitive(false);

        // We don't need to get result, because we connect buttons directly without needing to listen this.
        // Only X button is not handled, so we handle it now
        let _response_type = dialog_rules.run();
        dialog_rules.hide();
        window_main.set_sensitive(true);
    });
}
