# Szyszka

Szyszka is a simple but powerful and fast bulk file renamer.

<div align="center"><img src="https://github.com/user-attachments/assets/58e9e05e-8ab2-44fc-91a5-f5859427b7b7" alt="Szyszka" width="800" /></div>

## Features
- Great performance
- Available for Linux, Mac and Windows
- Gui created with Slint
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
Prebuilt binaries have no mandatory runtime dependencies on any platform (Linux, macOS, Windows).

## Installation
### Precompiled Binaries
Available at https://github.com/qarmin/szyszka/releases

### Cargo/Crates.io
https://crates.io/crates/szyszka
```
cargo install szyszka
```

## Alternatives
I tried to use different apps, but they didn't suit my needs.
- [Nautilus Renamer](https://launchpad.net/nautilus-renamer) - Quite fast, builtin into nautilus but hang when using it with >10k files and cannot be used with files/folders from different directories
- [Thunar Bulk Rename](https://docs.xfce.org/xfce/thunar/bulk-renamer/start) - Szyszka bases a lot of its features on this app, thunar bulk rename cannot add items recursivelly or rename folders.
- [Bulky](https://github.com/linuxmint/bulky) - simple, good looking and quite powerfull app, but slow and I had problems in install it

## Contribution
Contributions are very welcome - bug reports, pull requests, testing etc.   
When creating or modifying existing rules, don't forget about updating/adding tests!  
You can also add/improve translations in crowdin - https://crowdin.com/project/szyszka

## Name 
Szyszka is Polish word which means Pinecone.

Why such a strange name?

Would you remember another app name like Rename Files Ultra?  
Probably not.  
But will you remember name Szyszka?  
Well... probably also not, but when you hear this name, you will instantly think of this app.

## License
MIT

