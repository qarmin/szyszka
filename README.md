# Szyszka

Szyszka is simple but powerful batch file rename.

![Szyszka](https://user-images.githubusercontent.com/41945903/118101662-1e719480-b3d8-11eb-83d6-35e88fc919c5.png)
## Features
- Written in Rust
- Available for Linux, Max, Windows
- Very simple GUI created with GTK3
- Multiple rules which can be freely combined:
  - Replace text
  - Trim text
  - Adding text
  - Adding numbers
  - Purge text
  - Change letters to big/small
  - Use custom rule


## Requirements
### Linux
You need to install GTK(it should be available by default on most distributions) and the canberra-gtk-module.
```shell
sudo apt install libgtk3-dev libcanberra-gtk-module
```
### MacOS(not tested)
You need to install GTK from brew
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows(not tested)
Packed zip file contains all dependences, but if you want, you can install https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases and move file whatever you want.

## Installation
### Precompiled Binaries
Available in https://github.com/qarmin/szyszka/releases

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

## Future work
- Adding Regex support
- New icon
- Allow to add folders
- Recursive search of directories
- Saving/loading presets
- Reordering files/rules
- Trim x number of characters

## Contribution
Very welcome - bug reporting, pull requests, testing etc.  
If creating or modifing existing rule, don't forget about updating/adding tests!

## Name 
Szyszka is Polish word which means Pinecone

Why such strange name?

Would you remember another app name like Rename Files Ultra?  
Probably no.  
But will you remember name Szyszka?  
Well... probably also no, but when you heard this name instantly you will think about this app.

## Why?
I know that on Linux which I primarily use, there is a lot of good file renamers(and even more on Windows), but I coulnd't find any feasible for me.  
Available apps installs a lot of dependences, works slowly or just have very bloatet UI.  

## License
MIT
