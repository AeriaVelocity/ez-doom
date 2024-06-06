/// EzDOOM Configuration Utility
/// 
/// This file implements the configuration utility for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-06

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Align, Label, Stack};

/// Creates the config page for EzDOOM.
pub fn create_page(window: &ApplicationWindow, _stack: &Stack) -> gtk::Box {
    window.set_title(Some("EzDOOM Config"));
    
    let config_layout = gtk::Box::new(gtk::Orientation::Vertical, 18);
    config_layout.set_halign(Align::Start);

    let config_title = Label::new(Some("Config utility goes here."));
    config_layout.append(&config_title);

    config_layout
}
