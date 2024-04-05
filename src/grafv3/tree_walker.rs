use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use crate::AppState;
use crate::grafv3::file_counter::count_file;
use crate::grafv3::tree::{FolderFile, FolderValue};
use crate::grafv3::tree_walker::FolderLoCError::{LanguageNotFound, NoFileExtension};
use crate::grafzahl::counter::CountFileError;
use crate::grafzahl::counter::CountFileError::IoError;
use crate::grafzahl::ignore_checker;


pub(crate) fn folder_loc(path: PathBuf, state: &mut AppState) -> FolderFile {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());

    let ignored = ignore_checker::check_if_ignored(&path, &state);
    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap().to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();


    if ignored {
        return FolderFile {
            name,
            ignored: true,
            value: Ok(FolderValue::None),
        };
    };

    if path.is_file() {
        let ext = if let Some(a) = path.extension() { a } else {
            return FolderFile {
                name,
                ignored: false,
                value: Err(NoFileExtension),
            };
        }.to_str().expect("Unable to convert File extension into Unicode!");

        let Some(lang) = state.languages.iter().find(|&lang| lang.file_extension.eq(ext)) else {
            return FolderFile {
                name,
                ignored: false,
                value: Err(LanguageNotFound(ext.to_string())),
            };
        };

        let file = match File::open(&path) {
            Ok(v) => v,
            Err(e) => {
                return FolderFile {
                    name,
                    ignored,
                    value: Err(FolderLoCError::IoError(e)),
                };
            }
        };
        let Ok(count) = count_file(file, lang) else {
            return FolderFile {
                name,
                ignored,
                value: Err(FolderLoCError::EncodingNotSupported),
            };
        };

        return FolderFile {
            name,
            ignored: false,
            value: Ok(FolderValue::File(count)),
        };
    }

    //TODO count folder foreach


    todo!()
}


pub(crate) enum FolderLoCError {
    FileFolderMissingName,
    NoFileExtension,
    FileFolderNameIsNotUnicode,
    LanguageNotFound(String),
    EncodingNotSupported,
    IoError(io::Error),
}

impl From<io::Error> for CountFileError {
    fn from(value: io::Error) -> Self {
        return IoError(value);
    }
}


// pub(crate) fn language_loc(path: PathBuf, state: &mut AppState) -> Vec<LanguageCount> {}
