# Szyszka

Szyszka is a simple but powerful and fast bulk file renamer.

![Szyszka](https://user-images.githubusercontent.com/41945903/126200297-e0552164-2970-449f-9e68-bd47d231e041.png)
## Features
- Great performance
- Available for Linux, Mac and Windows
- Gui created with GTK 4
- Multiple rules which can be freely combined:
  - Replace text
  - Trim text
  - Add text
  - Add numbers
  - Purge text
  - Change letters to upper/lowercase
  - Custom rules
- Save rules to be able to use them later
- Ability to edit, reorder rules and results
- Handle even hundreds thousands of records

## Requirements
### Linux
You need to install GTK 4 (it should be available by default on most distributions), which is bundled in Ubuntu 22.04 and newer.
```shell
sudo apt install libgtk-4-bin
```
### MacOS (not tested)
You need to install GTK using brew
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows
The released zip file contains all dependencies, so it work out of the box on Windows 10+;

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

## Alternatives
I tried to use different apps, but they didn't suit my needs.
- [Nautilus Renamer](https://launchpad.net/nautilus-renamer) - Quite fast, builtin into nautilus but hang when using it with >10k files and cannot be used with files/folders from different directories
- [Thunar Bulk Rename](https://docs.xfce.org/xfce/thunar/bulk-renamer/start) - Szyszka bases a lot of its features on this app, thunar bulk rename cannot add items recursivelly or rename folders.
- [Bulky](https://github.com/linuxmint/bulky) - simple, good looking and quite powerfull app, but slow and I had problems in install it

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

## License
MIT

