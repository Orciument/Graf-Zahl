use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;
use std::ops::Add;
use std::path::PathBuf;

use quicli::prelude::error;

use crate::grafzahl::counter::{Count, count_file, CountFileError};
use crate::grafzahl::tree_indexer::FolderElement::*;

pub(crate) struct Folder {
    pub(crate) name: String,
    pub(crate) members: Vec<FolderElement>,
    pub(crate) count: HashMap<String, Count>,
}

pub(crate) enum FolderElement {
    Folder(Folder),
    FolderEmpty,
    File(File),
}

pub(crate) struct File {
    pub(crate) name: String,
    pub(crate) extension: String,
    pub(crate) language: String,
    pub(crate) count: Count,
}

pub(crate) fn scan_directory(path: &PathBuf) -> FolderElement {
    if !path.is_absolute() {
        panic!("Received Filepath is not absolut! {}", &path.display())
    }

    if !path.exists() {
        panic!("No File/Folder exists at this Path: {}", &path.display())
    }

    //TODO Check if Path ends in "/" or ".."

    //TODO Add Ignore feature (here, so it is for files and folder
    //3 Ignore list cases
    //test -< Dir name
    //.test -< File Extension
    //test.test -< File Name
    //nvm i should copy git ignore


    if path.is_file() {
        //TODO Should be in count_file, count_file should return a File
        let name = get_name(&path);
        let extension = path.extension().unwrap_or("".as_ref())
            .to_str().unwrap().to_string();

        let counted_file = match count_file(path.clone()) {
            Ok(k) => k,
            Err(e) => {
                //TODO Add command line option to hide these
                error!("Counting of file failed! (Path: {}", path.display());
                match e {
                    CountFileError::LanguageNotFoundError =>
                    //TODO Remove multiple warnings for each filetype
                        println!("Language for this file was not found in the config! (extension:\"{}\"", extension),
                    CountFileError::IoError(io) =>
                        println!("{}", io)
                }
                return FolderEmpty;
            }
        };

        let file = File {
            name,
            extension,
            language: counted_file.1,
            count: counted_file.0,
        };
        return File(file);
    }

    //Path is folder
    let mut members: Vec<FolderElement> = Vec::new();

    //Find all Entries of the directory
    for element_result in path.read_dir().unwrap() {
        let ele_path = element_result.unwrap().path();
        let folder_element = scan_directory(&ele_path);
        match folder_element {
            FolderEmpty => continue,
            _ => members.push(folder_element)
        }
    }

    if members.len() == 0 {
        return FolderEmpty;
    }

    //Early return if the Folder only contains 1 File/Directory, because then we can just copy the values
    if members.len() == 1 {
        let count = match &members[0] {
            Folder(f) => f.count.clone(),
            FolderEmpty => return FolderEmpty,
            File(f) =>
                HashMap::from([(f.language.clone(), f.count)])
        };

        let folder = Folder {
            name: get_name(&path),
            members,
            count,
        };
        return Folder(folder);
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

    let folder = Folder {
        name: get_name(&path),
        members,
        count: total_map,
    };
    Folder(folder)
}


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
        Some(v) => { base.insert(language.clone(), add_counts(v, add)); }
    }
}

fn add_counts(a: &Count, b: &Count) -> Count {
    a.add(*b)
}