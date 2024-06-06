/// EzDOOM Installation Wizard
/// 
/// This file implements the installation wizard for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-06

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, ApplicationWindow, Align, Label, Stack, Button, CheckButton};

use crate::clone;

/// Creates the installer page for EzDOOM.
pub fn create_page(window: &ApplicationWindow, stack: &Stack) -> gtk::Box {
    window.set_title(Some("EzDOOM Installer"));
    window.set_width_request(800);
    window.set_height_request(600);

    let installer_layout = gtk::Box::new(gtk::Orientation::Vertical, 8);
    installer_layout.set_halign(Align::Center);
    installer_layout.set_valign(Align::Center);

    let label = Label::new(Some(
        "EzDOOM will now install DOOM to your computer.\n\
        Before we proceed, we'll need to ask you a few questions about your preferred setup.\n\
        Press Confirm to proceed with setup, or Cancel to exit."
    ));
    installer_layout.append(&label);

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    button_box.set_halign(Align::Center);
    installer_layout.append(&button_box);

    let confirm_button = Button::with_label("Confirm");
    confirm_button.connect_clicked(clone!(@weak window, @weak stack => move |_| {
        let source_port_page = create_source_port_page(&window, &stack);
        stack.add_named(&source_port_page, Some("source-port-page"));
        stack.set_visible_child_name("source-port-page");
        window.present();
    }));
    button_box.append(&confirm_button);

    let cancel_button = Button::with_label("Cancel");
    cancel_button.connect_clicked(clone!(@weak stack => move |_| {
        stack.set_visible_child_name("welcome-page");
    }));
    button_box.append(&cancel_button);

    installer_layout
}

fn create_source_port_page(window: &ApplicationWindow, _stack: &Stack) -> gtk::Box {
    window.set_title(Some("EzDOOM Installer - Source Port"));

    let source_port_layout = gtk::Box::new(gtk::Orientation::Vertical, 8);
    source_port_layout.set_halign(Align::Center);
    source_port_layout.set_valign(Align::Center);

    let label = Label::new(Some("Please choose your preferred source port."));
    source_port_layout.append(&label);

    let option_gzdoom = CheckButton::with_label("GZDoom - Best for mods and non-DOOM IWAD support");
    let option_prboom = CheckButton::with_label("PrBoom-Plus - Compromise between graphical fidelity and performance");
    let option_crispy = CheckButton::with_label("Crispy DOOM - Compromise between aesthetics, QoL and purity");
    let option_chocolate = CheckButton::with_label("Chocolate DOOM - Fully pure, closest to the original DOOM experience");

    source_port_layout.append(&option_gzdoom);
    source_port_layout.append(&option_prboom);
    source_port_layout.append(&option_crispy);
    source_port_layout.append(&option_chocolate);

    option_crispy.set_group(Some(&option_gzdoom));
    option_prboom.set_group(Some(&option_gzdoom));
    option_chocolate.set_group(Some(&option_gzdoom));

    let next_button = Button::with_label("Next");
    next_button.set_sensitive(false);
    next_button.connect_clicked(clone!(@weak window, @weak _stack => move |_| {
        window.close();
    }));

    let next_sensitivity= clone!(@weak next_button => move |button: &CheckButton| {
        if button.is_active() {
            next_button.set_sensitive(true);
        } else {
            next_button.set_sensitive(false);
        }
    });

    option_gzdoom.connect_toggled(next_sensitivity.clone());
    option_prboom.connect_toggled(next_sensitivity.clone());
    option_crispy.connect_toggled(next_sensitivity.clone());
    option_chocolate.connect_toggled(next_sensitivity.clone());

    source_port_layout.append(&next_button);

    source_port_layout
}