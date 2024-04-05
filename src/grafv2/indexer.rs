use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::AppState;
use crate::grafv2::tree::{Count, CountFileError, DirectoryMember, FolderError, LanguageCount, LanguageType};
use crate::grafv2::tree::CountFileError as CFE;
use crate::grafv2::tree::FolderElement as FE;
use crate::grafv2::tree::FolderError::FailedRead;
use crate::grafzahl::languages::Language;

pub(crate) fn index_directory(og_path: &PathBuf, override_ignored: bool, state: &AppState) -> DirectoryMember {
    let path = &og_path.canonicalize().expect(&*format!("Absolut Path could not be found for Path: {}", og_path.display()));
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());
    //TODO check if exists

    let ignored = state.ignore.matched(path.as_path(), path.is_dir()).is_ignore();
    let name = path.file_name().unwrap().to_str().unwrap().to_string();

    //IS File
    if path.is_file() {
        return DirectoryMember {
            name,
            ignored,
            member: FE::File(count_file(path, &state.languages)),
        };
    }

    //IS Folder
    if ignored && !override_ignored {
        return DirectoryMember {
            name,
            ignored,
            member: FE::Folder(Ok(vec![])),
        };
    }
    return DirectoryMember {
        name,
        ignored,
        member: FE::Folder(count_directory(path, override_ignored, &state)),
    };
}


fn count_file(path: &PathBuf, languages: &Vec<Language>) -> Result<LanguageCount, CountFileError> {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path! {}", &path.display());
    assert!(path.is_file(), "Path is not a File! {}", &path.display());

    let ext = path.extension().ok_or_else(|| { CFE::NoFileExtension })?.to_str().expect("Can't convert Filename into UTF-8 String!");

    let Some(lang) = languages.iter().find(|&lang| lang.file_extension.eq(ext)) else {
        return Err(CFE::LanguageNotFoundError(ext.to_string()));
    };

    let file = fs::File::open(&path)?;
    let mut line_data: Count = Default::default();

    let lines = BufReader::new(file).lines();
    for line_result in lines {
        let l = &line_result.or_else(|_| { Err(CFE::EncodingNotSupported) })?;

        if l.contains(&lang.comment_symbol) {
            line_data.comment_count += 1;
        } else if l.trim().is_empty() {
            line_data.empty_count += 1;
        } else {
            line_data.code_count += 1;
        }
    }

    Ok(LanguageCount {
        name: lang.name.clone(),
        count: line_data,
        l_type: LanguageType::General, //TODO get LangType
    })
}

fn count_directory(path: &PathBuf, override_ignore: bool, state: &AppState) -> Result<Vec<DirectoryMember>, FolderError> {
    let mut members = Vec::new();

    for element_result in path.read_dir().or_else(|e| Err(FailedRead(e)))? {
        let ele_path = element_result.unwrap().path();
        members.push(index_directory(&ele_path, override_ignore, &state))
    }
    return Ok(members);
}