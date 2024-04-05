use std::{fs, io};
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::PathBuf;

use crate::grafzahl::counter::CountFileError::{EncodingNotSupported, IoError, LanguageNotFoundError, NoFileExtension};
use crate::grafzahl::languages::Language;
use crate::grafzahl::tree_indexer::File;

/// Holds the three different Counts for a Folder or File
#[derive(Debug, Copy, Clone, Default)]
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

#[derive(Debug)]
pub enum CountFileError {
    LanguageNotFoundError(String),
    NoFileExtension,
    EncodingNotSupported,
    IoError(io::Error),
}

impl From<io::Error> for CountFileError {
    fn from(value: io::Error) -> Self {
        return IoError(value);
    }
}

pub(crate) fn count_file(path: &PathBuf, languages: &Vec<Language>) -> Result<File, CountFileError> {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path! {}", &path.display());
    assert!(path.is_file(), "Path is not a File! {}", &path.display());

    let ext = path.extension().ok_or_else(|| { NoFileExtension })?.to_str().expect("Can't convert Filename into UTF-8 String!");

    let Some(lang) = languages.iter().find(|&lang| lang.file_extension.eq(ext)) else {
        return Err(LanguageNotFoundError(ext.to_string()));
    };

    let file = fs::File::open(&path)?;
    let mut line_data: Count = Default::default();

    let lines = BufReader::new(file).lines();
    for line_result in lines {
        let l = &line_result.or_else(|_| { Err(EncodingNotSupported) })?;

        if l.contains(&lang.comment_symbol) {
            line_data.comment_count += 1;
        } else if l.trim().is_empty() {
            line_data.empty_count += 1;
        } else {
            line_data.code_count += 1;
        }
    }

    Ok(File {
        //TODO This beauty should get a visit
        name: (path.file_name().unwrap().to_str().unwrap().to_string()).parse().unwrap(),
        language: lang.name.clone(),
        count: line_data,
    })
}
