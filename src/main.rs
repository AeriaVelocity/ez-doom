/// EzDOOM Main File
/// 
/// This file contains the main function and other necessary imports for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-05

use gtk4 as gtk;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Align, Label};
use glib::clone;

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

    let welcome_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    welcome_layout.set_halign(Align::Center);
    welcome_layout.set_valign(Align::Center);
    window.set_child(Some(&welcome_layout));

    let welcome_title = Label::new(Some("Welcome to EzDOOM!"));
    welcome_title.set_halign(Align::Center);

    let install_button = gtk::Button::with_label("Install DOOM"); 
    let config_button = gtk::Button::with_label("Configure EzDOOM");

    let close_button = gtk::Button::with_label("Close");
    close_button.connect_clicked(clone!(@weak window => move |_| window.close()));
    
    welcome_layout.append(&welcome_title);
    welcome_layout.append(&install_button);
    welcome_layout.append(&config_button);
    welcome_layout.append(&close_button);

    let installer_layout = gtk::Box::new(gtk::Orientation::Vertical, 18);
    installer_layout.set_halign(Align::Start);

    let installer_title = Label::new(Some("EzDOOM will now install DOOM to your computer.\nBefore we proceed, we'll need to ask you a few questions about your preferred setup."));

    installer_layout.append(&installer_title);

    let config_layout = gtk::Box::new(gtk::Orientation::Vertical, 18);
    config_layout.set_halign(Align::Start);

    let config_title = Label::new(Some("Config utility goes here."));

    config_layout.append(&config_title);

    install_button.connect_clicked(clone!(@weak window => move |_| window.set_child(Some(&installer_layout))));

    config_button.connect_clicked(clone!(@weak window => move |_| window.set_child(Some(&config_layout))));

    window.present();
}

/// It's the fricking main function. What did you really expect to see here?
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.aeriavelocity.EzDOOM")
        .build();

    app.connect_activate(on_activate);

    app.run()
}
