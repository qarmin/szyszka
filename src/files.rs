use chrono::{DateTime, Local};
use jwalk::WalkDir;
use rayon::prelude::*;
use std::cmp::{max, Ordering};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::time::UNIX_EPOCH;

#[derive(Default)]
pub struct ScanProgress {
    pub current: AtomicUsize,
    pub total: AtomicUsize,
}

#[cfg(not(target_family = "windows"))]
pub const CHARACTER: char = '/';
#[cfg(target_family = "windows")]
pub const CHARACTER: char = '\\';

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemStruct {
    pub full_name: String,
    pub name: String,
    pub future_name: String,
    pub path: String,
    pub size: u64,
    pub modification_date: u64,
    pub creation_date: u64,
    pub date: String,
    pub is_dir: bool,
}

#[derive(Default)]
pub struct ResultEntries {
    pub files: BTreeSet<String>,
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn enumerate_folder_contents(folders_to_check: Vec<PathBuf>, check_folders_inside: bool, ignore_folders: bool) -> Vec<PathBuf> {
    let mut new_entries = Vec::new();

    let mut folders = if check_folders_inside {
        for folder in folders_to_check {
            for entry in WalkDir::new(folder).skip_hidden(true).into_iter().filter_map(Result::ok) {
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
        new_entries
    } else {
        folders_to_check
    };

    folders.sort_by(|a, b| {
        let (path_a, name_a) = split_path(a);
        let (path_b, name_b) = split_path(b);
        let res = path_a.cmp(&path_b);
        if res == Ordering::Equal {
            return name_a.cmp(&name_b);
        }
        res
    });

    folders
}

pub fn sort_files(mut files: Vec<PathBuf>) -> Vec<PathBuf> {
    files.sort_by(|a, b| {
        let (path_a, name_a) = split_path(a);
        let (path_b, name_b) = split_path(b);
        let res = path_a.cmp(&path_b);
        if res == Ordering::Equal {
            return name_a.cmp(&name_b);
        }
        res
    });
    files
}

pub fn collect_files_async(items_to_check: Vec<PathBuf>, dedup: &BTreeSet<String>, progress: &Arc<ScanProgress>) -> Vec<ItemStruct> {
    progress.total.store(items_to_check.len(), AtomicOrdering::Relaxed);
    progress.current.store(0, AtomicOrdering::Relaxed);
    let timezone_offset = Local::now().offset().local_minus_utc();

    items_to_check
        .into_par_iter()
        .map(|file_entry| {
            let result = process_one_item(&file_entry, dedup, timezone_offset);
            progress.current.fetch_add(1, AtomicOrdering::Relaxed);
            result
        })
        .filter_map(|t| t)
        .collect()
}

fn process_one_item(file_entry: &Path, dedup: &BTreeSet<String>, timezone_offset: i32) -> Option<ItemStruct> {
    let (path, name) = split_path(file_entry);
    // Dedup on the canonical path so the same file referenced via a different (relative or
    // symlinked) string is recognised as a duplicate. `dedup` is populated with canonical paths.
    let canonical = file_entry.canonicalize().ok()?.to_string_lossy().to_string();
    if dedup.contains(&canonical) {
        return None;
    }

    let file_metadata = fs::metadata(file_entry).ok()?;

    let size = file_metadata.len();
    let modification_date = file_metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64);
    let creation_date = file_metadata
        .created()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64);

    Some(ItemStruct {
        full_name: canonical,
        name: name.clone(),
        future_name: name,
        path,
        size,
        modification_date,
        creation_date,
        date: DateTime::from_timestamp(creation_date as i64, 0)
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default(),
        is_dir: file_metadata.is_dir(),
    })
}

pub fn regex_check(expression: &str, directory: impl AsRef<Path>) -> bool {
    let temp_splits: Vec<&str> = expression.split('*').collect();
    let mut splits: Vec<&str> = Vec::new();
    for i in temp_splits {
        if !i.is_empty() {
            splits.push(i);
        }
    }
    if splits.is_empty() {
        return false;
    }

    let directory = directory.as_ref().to_string_lossy();

    for split in &splits {
        if !directory.contains(split) {
            return false;
        }
    }

    let mut position_of_splits: Vec<usize> = Vec::new();

    let Some(first_index) = directory.find(splits[0]) else {
        return false;
    };
    if !expression.starts_with('*') && first_index > 0 {
        return false;
    }
    if let Some(last) = splits.last() {
        if !expression.ends_with('*') && !directory.ends_with(last) {
            return false;
        }
    }

    position_of_splits.push(first_index);
    let mut current_index: usize;
    let mut found_index: usize;
    for i in splits[1..].iter().enumerate() {
        let Some(prev) = position_of_splits.get(i.0) else {
            return false;
        };
        current_index = *prev + i.1.len();
        if current_index > directory.len() {
            return false;
        }
        let Some(rest) = directory.get(current_index..) else {
            return false;
        };
        let Some(t) = rest.find(i.1) else {
            return false;
        };
        found_index = t;
        position_of_splits.push(found_index + current_index);
    }
    true
}
