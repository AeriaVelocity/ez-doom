/// EzDOOM Main File
/// 
/// This file contains the main function and other necessary imports for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-05

use gtk4 as gtk;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

/// It's the fricking main function. What did you really expect to see here?
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.aeriavelocity.EzDOOM")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EzDOOM")
            .default_width(800)
            .default_height(600)
            .build();

        window.present();
    });

    app.run()
}
