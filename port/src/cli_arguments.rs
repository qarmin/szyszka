use std::path::{Path, PathBuf};
use std::process;

use log::error;

#[derive(Copy, Clone, Eq, PartialEq)]
enum SearchMode {
    Normal,
    Recursive,
    RecursiveSkipFolders,
}

#[derive(Default)]
pub struct CliPaths {
    pub files: Vec<PathBuf>,
    pub folders_normal: Vec<PathBuf>,
    pub folders_recursive: Vec<PathBuf>,
    pub folders_recursive_skip: Vec<PathBuf>,
}

impl CliPaths {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.folders_normal.is_empty() && self.folders_recursive.is_empty() && self.folders_recursive_skip.is_empty()
    }
}

#[expect(clippy::print_stdout)]
pub fn handle_help_version(arguments: &[String]) {
    let Some(second) = arguments.get(1) else {
        return;
    };
    if second == "-h" || second == "--help" {
        println!("Usage: szyszka [OPTION]... [FILE]...");
        println!("Szyszka is simple utility to rename files and folders");
        println!();
        println!("  -h, --help     display this help and exit");
        println!("  -v, --version  output version information and exit");
        println!("  -r             search folders recursively (skip directories)");
        println!("  -n             search folders non-recursively");
        println!("  -f             search folders recursively (include directories)");
        println!();
        println!("Examples:");
        println!("  szyszka");
        println!("  szyszka /home/user/Downloads");
        println!("  szyszka -r /home/user/Downloads");
        process::exit(0);
    } else if second == "-v" || second == "--version" {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }
}

pub fn parse_cli_paths(arguments: &[String]) -> CliPaths {
    let mut paths = CliPaths::default();
    let mut current_mode = SearchMode::Normal;

    for arg in arguments.iter().skip(1) {
        if arg == "-r" {
            current_mode = SearchMode::RecursiveSkipFolders;
        } else if arg == "-n" {
            current_mode = SearchMode::Normal;
        } else if arg == "-f" {
            current_mode = SearchMode::Recursive;
        } else {
            let Ok(path) = Path::new(arg).canonicalize() else {
                error!("Skipping invalid path: {arg}");
                continue;
            };
            if path.is_dir() {
                match current_mode {
                    SearchMode::Normal => paths.folders_normal.push(path),
                    SearchMode::Recursive => paths.folders_recursive.push(path),
                    SearchMode::RecursiveSkipFolders => paths.folders_recursive_skip.push(path),
                }
            } else if path.is_file() {
                paths.files.push(path);
            } else {
                error!("Error: {arg} is not a valid file or folder");
            }
        }
    }
    paths
}
