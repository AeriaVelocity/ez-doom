# EzDOOM

EzDOOM (Easy DOOM) is a Rust/GTK utility which allows you to quickly and easily
install and set up DOOM on GNU/Linux systems through the Flatpak package
manager.

EzDOOM was created out of frustration with the Flatpak version of GZDoom and
how needlessly complicated it is to get set up with mods and all that. EzDOOM
aims to solve this through a simple installation wizard and a simple
configuration utility.

## Requirements

### Core

- [ ] Write an installation wizard
- [ ] Allow for easy installation of GZDoom through Flatpak
  - [ ] Set up Flatpak permissions automagically
  - [ ] Create a Mods folder, if the user wants it
  - [ ] Optionally ask the user where their DOOM.WAD and DOOM2.WAD are stored
- [ ] Automatically download Freedoom Phase 1+2, if the user wants it

### Extra

- [ ] Add a CLI
- [ ] Add support for Crispy DOOM and Chocolate DOOM
- [ ] Add a configuration utility
- [ ] Add several install/config presets
- [ ] Modularity
- [ ] Upload EzDOOM to Flathub at some point
- [ ] Add support specifically for the Steam Deck
