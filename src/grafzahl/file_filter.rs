#![deprecated]

use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::grafzahl::language_counter::LANGUAGES;

static SUPPORTED_EXT: Lazy<Vec<&str>> = Lazy::new(|| {
    //Copy just the File Extensions to a new Vec
    LANGUAGES.iter().fold(vec![], |mut acc, x| {
        acc.push(x.file_extension);
        acc
    })
});

pub fn filter_files(vec: &mut Vec<PathBuf>) {
    vec.retain(file_is_supported);
}

fn file_is_supported(p: &PathBuf) -> bool {
    let ext = match p.extension() {
        None => return false,
        Some(x) => match x.to_str() {
            None => return false,
            Some(y) => y,
        },
    };
    SUPPORTED_EXT.contains(&ext)
}
