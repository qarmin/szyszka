# Szyszka

Szyszka is simple but powerful file rename.

It is multiplatform and runs

Allows user to easily create rules which will be used to step by step rename files

![Szyszka](https://user-images.githubusercontent.com/41945903/103483978-732e4e80-4deb-11eb-8cc0-a0d5d7be90fb.png)

## Features
Szyszka:
- Is written in Rust
- Is available for Linux, Max, Windows  
- Have multiple rules to execute on name, extension or both:
  - Replace text
  - Trim text
  - Adding text
  - Adding numbers  
  - Purge text
  - Change size of text(Upper/Lower cases)
  - Use custom rule





## Future work
- Adding Regex support


## TODO ADD ICON
## Requirements
### Linux
You need to install GTK(it should be available by default on most distributions)
```
sudo apt install libgtk3-dev
```
### MacOS
You need to install GTK from brew
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust gtk+3
```

### Windows
Packed zip file contains all dependences, but if you want, you can install https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/releases and move file whatever you want.

## Status
For now Szyszka

I want to add before 1.0 release:
- Multiple Rules:
  - Uppercase/Downcase names
  - Search and Replace
  - Add text(date, file size, modification date)
  - Remove specific text
  - Add specific number(01,02,001,002 etc.)
  - Maybe Regex support(unlikely in short term)
  - Append folder name
- Better UI
- Adding entire folders

## Name 
Szyszka is Polish word which means Conifer Cone
