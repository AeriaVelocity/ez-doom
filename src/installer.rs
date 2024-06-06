/// EzDOOM Installation Wizard
/// 
/// This file implements the installation wizard for EzDOOM.
/// 
/// Author: Arsalan "Aeria" Kazmi <sonicspeed848@gmail.com>
/// Created: 2024-06-06

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Align, Label, Stack};

enum InstallerStage {
    Preinstall,
    SourcePort,
    ModSupport,
    ModsFolder,
    TerminalAccess,
    IWADs,
    SelectIWADsDirectory,
    ConfirmChanges,
    Installing,
    Finish
}

/// Creates the installer page for EzDOOM.
pub fn create_page(window: &ApplicationWindow, _stack: &Stack) -> gtk::Box {
    window.set_title(Some("EzDOOM Installer"));

    let mut installer_stage = InstallerStage::Preinstall;

    match installer_stage {
        InstallerStage::Preinstall => create_preinstall_page(),
        InstallerStage::SourcePort => create_source_port_page(),
        InstallerStage::ModSupport => create_mod_support_page(),
        InstallerStage::ModsFolder => create_mods_folder_page(),
        InstallerStage::TerminalAccess => create_terminal_access_page(),
        InstallerStage::IWADs => create_iwads_page(),
        InstallerStage::SelectIWADsDirectory => create_select_iwads_directory_page(),
        InstallerStage::ConfirmChanges => create_confirm_changes_page(),
        InstallerStage::Installing => create_installing_page(),
        InstallerStage::Finish => create_finish_page(),
    } 
}

fn create_preinstall_page() -> gtk::Box {
    let installer_layout = gtk::Box::new(gtk::Orientation::Vertical, 6);
    installer_layout.set_halign(Align::Center);
    installer_layout.set_margin_top(32);

    let installer_title = Label::new(Some("EzDOOM will now install DOOM to your computer.\nBefore we proceed, we'll need to ask you a few questions about your preferred setup."));
    installer_layout.append(&installer_title);

    installer_layout
}

// These are placeholders for now
fn create_source_port_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_mod_support_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_mods_folder_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_terminal_access_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_iwads_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_select_iwads_directory_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_confirm_changes_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_installing_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
fn create_finish_page() -> gtk::Box { gtk::Box::new(gtk::Orientation::Vertical, 0) }
