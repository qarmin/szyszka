use crate::add_files_folders::{add_files_to_check, add_folders_to_check};
use crate::fls;
use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::get_list_store_from_tree_view;
use crate::localizer::generate_translation_hashmap;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SearchMode {
    Normal,
    Recursive,
    RecursiveSkipFolders,
}

pub fn parse_cli_help_version_arguments(arguments: &[String]) {
    let Some(second_argument) = arguments.get(1) else {
        return;
    };

    if second_argument == "-h" || second_argument == "--help" {
        println!("Usage: szyszka [OPTION]... [FILE]...");
        println!("Szyszka is simple utility to rename files and folders");
        println!();
        println!("  -h, --help     display this help and exit");
        println!("  -v, --version  output version information and exit");
        println!("  -r             search folders recursively");
        println!("  -n             search folders non-recursively");
        println!("  -f             search folders recursively and skip folders");
        println!();
        println!("Examples:");
        println!("  szyszka");
        println!("  szyszka /home/user/Downloads");
        println!("  szyszka -r /home/user/Downloads");
        println!("  szyszka -i /home/user/Downloads -r /home/user/Pictures /home/user/Music");
        process::exit(0);
    } else if second_argument == "-v" || second_argument == "--version" {
        println!(
            "{package_name} {package_version}",
            package_name = env!("CARGO_PKG_NAME"),
            package_version = env!("CARGO_PKG_VERSION")
        );
        process::exit(0);
    }
}

pub fn parse_cli_arguments(gui_data: &GuiData, arguments: &[String]) {
    // println!("{arguments}");

    let mut files_to_add: Vec<PathBuf> = Vec::new();
    let mut folders_to_add_recursive_skip_folders: Vec<PathBuf> = Vec::new();
    let mut folders_to_add_recursive: Vec<PathBuf> = Vec::new();
    let mut folders_to_add: Vec<PathBuf> = Vec::new();
    let mut current_mode: SearchMode = SearchMode::Normal;

    for i in arguments.iter().skip(1) {
        if i == "-r" {
            current_mode = SearchMode::RecursiveSkipFolders;
        } else if i == "-n" {
            current_mode = SearchMode::Normal;
        } else if i == "-f" {
            current_mode = SearchMode::Recursive;
        } else {
            let Ok(path) = Path::new(i).canonicalize() else {
                continue
            };
            if path.is_dir() {
                match current_mode {
                    SearchMode::Normal => {
                        folders_to_add.push(path.clone());
                    }
                    SearchMode::RecursiveSkipFolders => {
                        folders_to_add_recursive_skip_folders.push(path.clone());
                    }
                    SearchMode::Recursive => {
                        folders_to_add_recursive.push(path.clone());
                    }
                }
            } else if path.is_file() {
                files_to_add.push(path.clone());
            } else {
                eprintln!("Error: {i} is not valid file or folder");
            }
        }
    }

    let tree_view_results = gui_data.results.tree_view_results.clone();
    let list_store = get_list_store_from_tree_view(&tree_view_results);
    let result_entries = gui_data.shared_result_entries.clone();
    let mut result_entries = result_entries.borrow_mut();

    add_files_to_check(files_to_add, &list_store, &mut result_entries);
    add_folders_to_check(folders_to_add, &list_store, &mut result_entries, false, false);
    add_folders_to_check(folders_to_add_recursive, &list_store, &mut result_entries, true, false);
    add_folders_to_check(folders_to_add_recursive_skip_folders, &list_store, &mut result_entries, true, true);

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();
    label_files_folders.set_text(&fls!(
        "upper_files_folders_label_up_to_date",
        generate_translation_hashmap(vec![("files_number", result_entries.files.len().to_string())])
    ));
}
