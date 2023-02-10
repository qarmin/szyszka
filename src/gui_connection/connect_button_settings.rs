use glib::signal::Inhibit;
use gtk4::prelude::*;

use crate::GuiData;

pub fn connect_button_settings(gui_data: &GuiData) {
    let button_setting = gui_data.upper_buttons.button_setting.clone();
    let window_settings = gui_data.settings.window_settings.clone();
    button_setting.connect_clicked(move |_e| {
        window_settings.show();
    });

    let window_settings = gui_data.settings.window_settings.clone();
    window_settings.connect_close_request(|window_settings| {
        window_settings.hide();
        Inhibit(true)
    });
}
