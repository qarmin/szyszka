use crate::GuiData;
use gtk::prelude::*;

pub fn connect_button_settings(gui_data: &GuiData) {
    let button_setting = gui_data.upper_buttons.button_setting.clone();
    let window_settings = gui_data.settings.window_settings.clone();
    button_setting.connect_clicked(move |e| {
        window_settings.show();
    });

    let window_settings = gui_data.settings.window_settings.clone();
    window_settings.connect_delete_event(|window_settings, _| {
        window_settings.hide();
        Inhibit(true)
    });
}
