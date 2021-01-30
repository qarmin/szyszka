use crate::class_gui_data::GuiData;
use crate::example_fields::update_examples;
use gtk::ButtonExt;

pub fn connect_rule_window_purge_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();
    let radio_button_purge_both = gui_data.window_rules.purge.radio_button_purge_both.clone();
    let radio_button_purge_name = gui_data.window_rules.purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = gui_data.window_rules.purge.radio_button_purge_extension.clone();

    radio_button_purge_extension.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_purge_name.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_purge_both.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
}
