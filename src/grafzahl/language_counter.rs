use std::collections::HashMap;
use std::fs::{File, read};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use crate::grafzahl::file_filter::{get_language_info, Language};

#[derive(Debug, Clone)]
pub struct LineData {
    language: String,
    comment_count: u32,
    code_count: u32,
    empty_count: u32,
}

pub fn count_files(paths: Vec<PathBuf>) {
    let mut map: HashMap<String, LineData> = HashMap::new();

    //TODO Threads
    for f in paths {
        let counts = count_lines(f);

        if map.contains_key(&*counts.language) {
            let lang = match map.get_mut(&*counts.language) {
                None => continue,
                Some(x) => x
            };
            lang.code_count += counts.code_count;
            lang.empty_count += counts.empty_count;
            lang.comment_count += counts.comment_count;
        } else {
            map.insert(counts.language.clone(), counts);
        }
    }

    for value in map.values() {
        println!("{} => {} (LoC: {}, Comment: {}, NewLines: {})",
                 value.language,
                 (value.code_count + value.comment_count + value.empty_count),
                 value.code_count,
                 value.comment_count,
                 value.empty_count);
    }
}

pub fn count_lines(path: PathBuf) -> LineData {
    let mut line_data = LineData {
        language: "".to_string(),
        comment_count: 0,
        code_count: 0,
        empty_count: 0,
    };

    let lang = match get_language_info(&path) {
        None => return line_data,
        Some(x) => x,
    };
    line_data.language = lang.name.to_string();

    let file = match File::open(path) {
        Ok(x) => x,
        Err(_) => { return line_data }
    };

    let lines = BufReader::new(file).lines();
    for l_opt in lines {
        let l = match l_opt {
            Ok(x) => x,
            Err(_) => continue,
        };

        if l.contains(&lang.comment_symbol) {
            line_data.comment_count += 1;
            continue;
        }

        else if l.trim().len() == 0 {
            line_data.empty_count += 1;
        }

        else {
            line_data.code_count += 1;
        }
    }

    line_data
}