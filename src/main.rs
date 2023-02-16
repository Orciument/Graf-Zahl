use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use Graf_Zahl::grafzahl::language_counter::{count_files, count_lines};
use Graf_Zahl::grafzahl::file_filter::filter_files;
use crate::grafzahl::package_indexer::search_files;

mod grafzahl;


fn main() {
    // let path = PathBuf::from("C:\\Users\\Master\\IdeaProjects\\Graf-Zahl");
    let path = PathBuf::from("./../JavaTwitchBot");
    let mut vec = search_files(path).unwrap();
    let filtered = filter_files(&vec);
    count_files(filtered);
}