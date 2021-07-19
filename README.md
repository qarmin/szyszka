# Szyszka

Szyszka is a simple but powerful and fast bulk file renamer.

![Szyszka](https://user-images.githubusercontent.com/41945903/126200297-e0552164-2970-449f-9e68-bd47d231e041.png)
## Features
- Written in Rust
- Available for Linux, Mac and Windows
- Simple GUI created using GTK3
- Multiple rules which can be freely combined:
  - Replace text
  - Trim text
  - Add text
  - Add numbers
  - Purge text
  - Change letters to upper-/lowercase
  - Custom rules
- Ability to edit, reorder rules and results
- Handle even hundreds thousands of records

## Requirements
### Linux
You need to install GTK (it should be available by default on most distributions) and the canberra-gtk-module.
```shell
sudo apt install libgtk3-dev libcanberra-gtk-module
```
### MacOS (not tested)
You need to install GTK using brew
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows
The released zip file contains all dependencies, so it work on Windows 7 SP1+.  
If you want to, you can install the GTK runtime from https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases, ensure that its environment variables are set properly and run Szyszka from anywhere.

## Installation
### Precompiled Binaries
Available at https://github.com/qarmin/szyszka/releases

### Snap
https://snapcraft.io/szyszka  
```
snap install szyszka
sudo snap connect szyszka:removable-media # Allows to see files on external devices
```

### Flatpak
TODO

### Cargo/Crates.io
https://crates.io/crates/szyszka
```
cargo install szyszka
```

### Gentoo Linux
szyszka is available on Gentoo's GURU overlay
```
emerge -av gui-apps/szyszka
```

## Future work
- Adding Regex support
- Saving/loading presets
- Trim x number of characters

## Contribution
Contributions are very welcome - bug reports, pull requests, testing etc.   
When creating or modifying existing rules, don't forget about updating/adding tests!

## Name 
Szyszka is Polish word which means Pinecone.

Why such a strange name?

Would you remember another app name like Rename Files Ultra?  
Probably not.  
But will you remember name Szyszka?  
Well... probably also not, but when you hear this name, you will instantly think of this app.

## Why?
I know that on Linux, which I primarily use, there is a lot of good file renamers (and even more on Windows), but I couldn't find any that would suit my needs.
Available apps install a lot of dependencies, work slowly or just have a very bloated UI.  

If you want very simple apps without too much of features, look at [Bulky](https://github.com/linuxmint/bulky), [Thunar Bulk Rename](https://docs.xfce.org/xfce/thunar/bulk-renamer/start) or [Nautilus Renamer](https://launchpad.net/nautilus-renamer).

## License
MIT

