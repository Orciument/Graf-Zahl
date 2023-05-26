use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
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

impl Add for Count {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            comment_count: self.comment_count + other.comment_count,
            code_count: self.code_count + other.code_count,
            empty_count: self.empty_count + other.empty_count,
        }
    }
}

pub(crate) static LANGUAGES: Lazy<Vec<Language>> = Lazy::new(import_languages);


#[derive(Debug)]
pub enum CountFileError {
    LanguageNotFoundError,
    IoError(std::io::Error),
}

pub fn count_file(path: PathBuf) -> Result<(Count, String), CountFileError> {
    //TODO Check if Path is really a file

    let lang = match get_lang(&path) {
        None => return Err(CountFileError::LanguageNotFoundError),
        Some(x) => x,
    };

    let file = match File::open(&path) {
        Ok(x) => x,
        Err(e) => return Err(CountFileError::IoError(e)),
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
        //TODO Add different counting options

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

    Ok((line_data, lang.name.clone()))
}

fn get_lang(p: &Path) -> Option<&Language> {
    let ext = p.extension()?.to_str()?;
    LANGUAGES.iter().find(|&lang| lang.file_extension.eq(ext))
}
