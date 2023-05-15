use gtk4::{Entry, MenuButton, Orientation};



use gtk4::prelude::*;

use crate::config::{load_custom_rules};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_window_rules_open(gui_data: &GuiData) {
    let custom = gui_data.window_rules.custom.clone();

    gui_data.window_rules.window_with_rules.connect_show(move |_e| {
        let entry_custom_text_to_change = custom.entry_custom_text_to_change.clone();
        let menu_button_load_custom_rule = custom.menu_button_load_custom_rule.clone();
        let cached_rules = load_custom_rules();
        let mut cached_borrowed = custom.imported_custom_rules.borrow_mut();
        *cached_borrowed = cached_rules;

        if !cached_borrowed.is_empty() {
            create_custom_popovers(&menu_button_load_custom_rule, &entry_custom_text_to_change, &cached_borrowed);
        }
    });
}

pub fn create_custom_popovers(menu_button_load_custom_rule: &MenuButton, entry_custom_text_to_change: &Entry, cached_items: &[String]) {
    let popover = gtk4::Popover::builder().build();
    let new_box = gtk4::Box::builder().orientation(Orientation::Vertical).build();
    for item in cached_items {
        let button = gtk4::Button::builder().label(item).build();
        let popover_clone = popover.clone();
        let entry_custom_text_to_change = entry_custom_text_to_change.clone();
        button.connect_clicked(move |e| {
            entry_custom_text_to_change.set_text(&e.label().unwrap());
            popover_clone.hide();
        });

        new_box.append(&button);
    }
    popover.set_child(Some(&new_box));
    menu_button_load_custom_rule.set_popover(Some(&popover));
}
