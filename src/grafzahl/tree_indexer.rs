use std::collections::HashMap;
use std::path::PathBuf;

use quicli::prelude::error;

use crate::AppState;
use crate::grafzahl::counter::{Count, count_file, CountFileError};
use crate::grafzahl::ignore_checker;
use crate::grafzahl::tree_indexer::FolderElement::*;

pub(crate) struct Folder {
    pub(crate) name: String,
    pub(crate) members: Vec<FolderElement>,
    pub(crate) count: HashMap<String, Count>,
}

impl Folder {
    pub(crate) fn total_count(&self) -> Count {
        let mut total: Count = Count {
            comment_count: 0,
            code_count: 0,
            empty_count: 0,
        };
        for value in self.count.values() {
            total.comment_count += value.comment_count;
            total.code_count += value.code_count;
            total.empty_count += value.empty_count;
        }
        total
    }

    pub(crate) fn total(&self) -> u32 {
        let mut total: u32 = 0;
        for value in self.count.values() {
            total += value.empty_count + value.code_count + value.comment_count;
        }
        total
    }
}

pub(crate) enum FolderElement {
    Folder(Folder),
    FolderEmpty,
    File(File),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct File {
    pub(crate) name: String,
    pub(crate) language: String,
    pub(crate) count: Count,
}
/// # Important
/// This function can only be used on Paths that have been **canonicalize**'d, this is omitted
/// in this function to avoid the underling calls to the OP which may involve opening the
/// file to get the absolut path
pub(crate) fn scan_directory(path: &PathBuf, state: &mut AppState) -> FolderElement {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());

    if ignore_checker::check_if_ignored(&path, &state) {
        return FolderEmpty;
    };

    //Path is file
    if path.is_file() {
        match count_file(&path, &state.languages) {
            Ok(f) => { return FolderElement::File(f); }
            Err(CountFileError::NoFileExtension) => { /*Is ignored*/ }
            Err(CountFileError::LanguageNotFoundError(ext)) => {
                state.missing_lang.insert(ext);
            }
            Err(CountFileError::EncodingNotSupported) => {
                println!("File did not contain Valid UTF-8: \"{:?}\"", &path.display());
            }
            Err(CountFileError::IoError(io)) => {
                error!("Counting of file failed! (Path: \"{}\") Err: {}", path.display(), io);
            }
        };
        return FolderEmpty;
    }

    //Path is folder
    let mut members: Vec<FolderElement> = Vec::new();

    //Find all Entries of the directory
    for element_result in path.read_dir().expect(&*format!("Failed to read directory: {:?}", path)) {
        let ele_path = element_result.unwrap().path();
        members.push(scan_directory(&ele_path, state))
    }

    //Early return if Folder has no members
    if members.len() == 0 {
        return FolderEmpty;
    }

    //Early return if the Folder only contains 1 File/Directory, because then we can just copy the values
    if members.len() == 1 {
        let count = match &members[0] {
            FolderEmpty => return FolderEmpty,
            Folder(f) => f.count.clone(),
            File(f) => HashMap::from([(f.language.clone(), f.count)])
        };
        return Folder(Folder {
            name: get_name(&path),
            members,
            count,
        });
    }

    //calculate total for this Folder
    let mut total_map: HashMap<String, Count> = HashMap::new();
    for member in &members {
        match member {
            Folder(f) => merge_language_maps(&mut total_map, &f.count),
            File(f) => add_into_map(&mut total_map, &f.count, &f.language),
            FolderEmpty => {} //Nothing to add
        }
    }

    Folder(Folder {
        name: get_name(&path),
        members,
        count: total_map,
    })
}

/// This function can only be used when the Path was canonicalized beforehand, because otherwise
/// it could be that no name is found
//TODO Should be private
pub(crate) fn get_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_str().unwrap().to_string()
}

fn merge_language_maps(base: &mut HashMap<String, Count>, add: &HashMap<String, Count>) {
    for x in add {
        add_into_map(base, x.1, x.0);
    }
}

fn add_into_map(base: &mut HashMap<String, Count>, add: &Count, language: &String) {
    match base.get(language) {
        None => { base.insert(language.clone(), add.clone()); }
        Some(v) => { base.insert(language.clone(), *v + *add); }
    }
}