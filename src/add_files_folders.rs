use crate::help_function::{split_path, to_dir_file_name, to_dir_file_type, ColumnsResults, ResultEntries};
use chrono::{Local, NaiveDateTime};
use glib::ToValue;
use gtk4::ListStore;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::cmp::{max, Ordering};
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

pub struct ItemStruct {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modification_date: u64,
    pub creation_date: u64,
    pub date: String,
    pub is_dir: bool,
}

pub fn add_folders_to_check(folders_to_check: Vec<PathBuf>, list_store: &ListStore, result_entries: &mut ResultEntries, check_folders_inside: bool, ignore_folders: bool) {
    let mut new_entries = Vec::new();

    let mut folders;
    if check_folders_inside {
        for folder in folders_to_check {
            for entry in WalkDir::new(folder).follow_links(false).into_iter().filter_map(Result::ok) {
                if ignore_folders {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            new_entries.push(entry.path());
                        }
                    }
                } else {
                    new_entries.push(entry.path());
                }
            }
        }
        folders = new_entries;
    } else {
        folders = folders_to_check;
    }

    folders.sort_by(|a, b| {
        let (path_a, name_a) = split_path(a);
        let (path_b, name_b) = split_path(b);
        let res = path_a.cmp(&path_b);
        if res == Ordering::Equal {
            return name_a.cmp(&name_b);
        }
        res
    });

    let results = collect_files(&folders, result_entries);
    set_list_store_items(results, list_store);
}

pub fn add_files_to_check(mut files: Vec<PathBuf>, list_store: &ListStore, result_entries: &mut ResultEntries) {
    files.sort_by(|a, b| {
        let (path_a, name_a) = split_path(a);
        let (path_b, name_b) = split_path(b);
        let res = path_a.cmp(&path_b);
        if res == Ordering::Equal {
            return name_a.cmp(&name_b);
        }
        res
    });

    let results = collect_files(&files, result_entries);
    set_list_store_items(results, list_store);
}

fn set_list_store_items(results: Vec<ItemStruct>, list_store: &ListStore) {
    for result in results {
        let values: [(u32, &dyn ToValue); 8] = [
            (ColumnsResults::Type as u32, &(to_dir_file_type(result.is_dir) as u8)),
            (ColumnsResults::TypeString as u32, &to_dir_file_name(result.is_dir)),
            (ColumnsResults::CurrentName as u32, &result.name),
            (ColumnsResults::FutureName as u32, &result.name),
            (ColumnsResults::Path as u32, &result.path),
            (ColumnsResults::Size as u32, &result.size),
            (ColumnsResults::ModificationDate as u32, &result.modification_date),
            (ColumnsResults::CreationDate as u32, &result.creation_date),
        ];
        list_store.set(&list_store.append(), &values);
    }
}

fn collect_files(items_to_check: &[PathBuf], result_entries: &mut ResultEntries) -> Vec<ItemStruct> {
    let timezone_offset = Local::now().offset().local_minus_utc();

    let collected_items: Vec<_> = items_to_check
        .into_par_iter()
        .map(|file_entry| {
            let (path, name) = split_path(file_entry);
            let Some(full_name) = file_entry.to_str() else {
                println!("Failed to read name of {file_entry:?} (some characters may be missing in this name)");
                return None;
            };

            if result_entries.files.contains(full_name) {
                // Remove this println
                // println!("Already is used file name {}", full_name);
                return None; // There is already entry
            }

            //// Read Metadata
            let file_metadata = match fs::metadata(file_entry) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("Failed to load metadata of file {}, reason - \"{}\"", file_entry.display(), err);
                    return None;
                }
            };
            let size = file_metadata.len();
            let modification_date = match file_metadata.modified() {
                Ok(t) => {
                    if let Ok(d) = t.duration_since(UNIX_EPOCH) {
                        max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64
                    } else {
                        eprintln!("File {} seems to be modified before Unix Epoch.", file_entry.display());
                        0
                    }
                }
                Err(err) => {
                    eprintln!("Unable to get modification date from file {}, reason - \"{}\"", file_entry.display(), err);
                    0
                }
            };
            let creation_date = match file_metadata.created() {
                Ok(t) => {
                    if let Ok(d) = t.duration_since(UNIX_EPOCH) {
                        max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64
                    } else {
                        eprintln!("File {} seems to be created before Unix Epoch.", file_entry.display());
                        0
                    }
                }
                Err(err) => {
                    eprintln!("Unable to get creation date from file {}, reason - \"{}\"", file_entry.display(), err);
                    0
                }
            };

            Some(ItemStruct {
                name,
                path,
                size,
                modification_date,
                creation_date,
                date: NaiveDateTime::from_timestamp_opt(creation_date as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
                is_dir: file_metadata.is_dir(),
            })
        })
        .filter_map(|t| t)
        .collect();

    result_entries.files.extend(collected_items.iter().map(|t| t.name.clone()));

    collected_items
}
