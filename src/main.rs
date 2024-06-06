/// EzDOOM Main File
/// 
/// This file contains the main function and other necessary imports for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-05

use gtk4 as gtk;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Align, Label, Stack};
use glib::clone;

mod installer;
mod config;

/// Handles the window activation event.
/// 
/// This contains the actual functionality of EzDOOM and is called from the
/// GTK `activate` event through `connect_activate`.
fn on_activate(app: &gtk::Application) {
    let window_dimensions = (800, 600);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("EzDOOM")
        .default_width(window_dimensions.0)
        .default_height(window_dimensions.1)
        .build();

    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    window.set_child(Some(&stack));

    let welcome_page = create_welcome_page(&stack);
    stack.add_named(&welcome_page, Some("welcome-page"));

    let installer_page = installer::create_page(&window, &stack);
    stack.add_named(&installer_page, Some("installer-page"));

    let config_page = config::create_page(&window, &stack);
    stack.add_named(&config_page, Some("config-page"));

    window.present();
}

fn create_welcome_page(stack: &Stack) -> gtk::Box {
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    layout.set_halign(Align::Center);
    layout.set_valign(Align::Center);

    let title = Label::new(Some("Welcome to EzDOOM!"));
    title.set_halign(Align::Center);
    layout.append(&title);

    let install_button = gtk::Button::with_label("Install DOOM"); 
    install_button.connect_clicked(clone!(@weak stack => move |_| stack.set_visible_child_name("installer-page")));
    layout.append(&install_button);

    let config_button = gtk::Button::with_label("Configure EzDOOM");
    config_button.connect_clicked(clone!(@weak stack => move |_| stack.set_visible_child_name("config-page")));
    layout.append(&config_button);

    layout
}

/// It's the fricking main function. What did you really expect to see here?
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.aeriavelocity.EzDOOM")
        .build();

    app.connect_activate(on_activate);

    app.run()
}
