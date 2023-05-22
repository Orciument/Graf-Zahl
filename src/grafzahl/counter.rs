use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::grafzahl::languages::{import_languages, Language};

/// Holds the three different Counts for a Folder or File
#[derive(Debug, Copy, Clone)]
pub struct Count {
    pub comment_count: u32,
    pub code_count: u32,
    pub empty_count: u32,
}

pub(crate) static LANGUAGES: Lazy<Vec<Language>> = Lazy::new(import_languages);

pub fn count_project_files(files_vec: Vec<PathBuf>) -> HashMap<String, Count> {
    let mut map: HashMap<String, Count> = HashMap::new();

    //TODO Threads
    for f in files_vec {
        let counts = match count_file(f) {
            None => continue,
            Some(x) => x,
        };

        //If this Language was already encountered bevor we add it to the current Object,
        //rather than creating a duplicate one
        match map.get_mut(&counts.1) {
            None => {
                map.insert(counts.1, counts.0);
            }
            Some(lang) => {
                lang.code_count += counts.0.code_count;
                lang.empty_count += counts.0.empty_count;
                lang.comment_count += counts.0.comment_count;
            }
        };
    }
    map
}

pub fn count_file(path: PathBuf) -> Option<(Count, String)> {
    //TODO Check if Path is really a file

    let lang = match get_lang(&path) {
        None => return None,
        Some(x) => x,
    };

    let file = match File::open(&path) {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut line_data = Count {
        comment_count: 0,
        code_count: 0,
        empty_count: 0,
    };

    let lines = BufReader::new(file).lines();
    for l_opt in lines {
        let l = match l_opt {
            Ok(x) => x,
            Err(_) => continue,
        };

        // //Char Count Start
        // let char_count: u32 = l.len() as u32;
        // if l.contains(&lang.comment_symbol) {
        //     line_data.comment_count += &char_count;
        //     continue;
        // } else if l.trim().is_empty() {
        //     line_data.empty_count += &char_count;
        // } else {
        //     line_data.code_count += &char_count;
        // }
        // //Char Count End

        //Word Count Start
        let word_count: u32 = l.trim().split(' ').count() as u32;
        if l.contains(&lang.comment_symbol) {
            line_data.comment_count += &word_count;
            continue;
        } else if l.trim().is_empty() {
            line_data.empty_count += &word_count;
        } else {
            line_data.code_count += &word_count;
        }
        //Word Count End

        // if l.contains(&lang.comment_symbol) {
        //     line_data.comment_count += 1;
        //     continue;
        // } else if l.trim().is_empty() {
        //     line_data.empty_count += 1;
        // } else {
        //     line_data.code_count += 1;
        // }
    }

    Some((line_data, lang.name.clone()))
}

fn get_lang(p: &Path) -> Option<&Language> {
    let ext = p.extension()?.to_str()?;
    LANGUAGES.iter().find(|&lang| lang.file_extension.eq(ext))
}