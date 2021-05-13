# Szyszka

Szyszka is simple but powerful batch file rename.

![Szyszka](https://user-images.githubusercontent.com/41945903/118101662-1e719480-b3d8-11eb-83d6-35e88fc919c5.png)
## Features
- Written in Rust
- Available for Linux, Mac and Windows
- Very simple GUI created using GTK3
- Multiple rules which can be freely combined:
  - Replace text
  - Trim text
  - Add text
  - Add numbers
  - Purge text
  - Change letters to big/small
  - Use custom rule


## Requirements
### Linux
You need to install GTK (it should be available by default on most distributions)
```shell
sudo apt install libgtk3-dev
```
### MacOS (not tested)
You need to install GTK from brew
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows (not tested)
Packed zip file contains all dependencies, but if you want, you can install https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases and run file from anywhere.

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
- Allow adding folders
- Recursive search of directories
- Saving/loading presets
- Reordering files/rules
- Trim x number of characters

## Contribution
Contributions are very welcome - bug reports, pull requests, testing etc.  
When creating or modifing existing rules, don't forget to update/add tests!

## Name 
Szyszka is Polish word which means Pinecone.

Why such strange name?

Would you remember another app name like Rename Files Ultra?  
Probably not.  
But will you remember name Szyszka?  
Well... probably not, but when you hear this name you will think of this app.

## Why?
I know that on Linux, which I primarily use, there is a lot of good file renamers (even more on Windows), but I coulnd't find any that would suit my needs.  
Available apps install a lot of dependencies, work slowly or just have very bloated UI.  

## License
MIT
