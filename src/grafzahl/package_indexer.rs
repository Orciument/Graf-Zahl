use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::{CONFIG_LOCATION, get_config_location};

pub fn search_files(path: PathBuf) -> Option<Vec<PathBuf>> {
    let mut vec: Vec<PathBuf> = vec![];

    if path.is_file() {
        vec.push(path);
        return Some(vec);
    }

    if IGNORE_LIST.contains(&path.file_name()?.to_str()?.to_string()) {
        return None;
    }

    for f in fs::read_dir(path).ok()? {
        let entry = f.ok()?;
        let path = entry.path();
        if let Some(mut v) = search_files(path) {
            vec.append(v.as_mut());
        }
    }

    Some(vec)
}

static IGNORE_LIST: Lazy<Vec<String>> = Lazy::new(import_ignore_list);

fn import_ignore_list() -> Vec<String> {
    let path_string = format!("{}/ignore_list.txt", get_config_location());
    let path: PathBuf = PathBuf::from(&path_string);

    let file = match File::open(path) {
        Ok(x) => x,
        Err(e) => panic!("Could not find Ignore-List at: {path_string}! Might be missing privileges, or the Path to the File may be incorrect {e}")
    };

    let reader = BufReader::new(file);

    let mut vec: Vec<String> = vec![];

    for line in reader.lines() {
        vec.push(line.unwrap());
    }

    vec
}
