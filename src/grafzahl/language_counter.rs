use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use once_cell::sync::Lazy;

#[derive(Debug, Copy, Clone)]
pub struct LineData {
    pub comment_count: u32,
    pub code_count: u32,
    pub empty_count: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Language {
    pub name: &'static str,
    pub comment_symbol: &'static str,
    pub file_extension: &'static str,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.file_extension)
    }
}

pub(crate) static LANGUAGES: Lazy<Vec<Language>> = Lazy::new(|| {
    let x = vec![Language {
        name: "Rust",
        comment_symbol: "//",
        file_extension: "rs",
    }, Language {
        name: "Java",
        comment_symbol: "//",
        file_extension: "java",
    }, Language {
        name: "",
        comment_symbol: "",
        file_extension: "",
    }];
    x.iter().for_each(|x| { println!("{}",x) });
    x
});

pub fn count_project_files(files_vec: Vec<PathBuf>) -> HashMap<&'static str, LineData> {
    let mut map: HashMap<&str, LineData> = HashMap::new();

    //TODO Threads
    for f in files_vec {
        let counts = match count_lines(f) {
            None => continue,
            Some(x) => x
        };

        //If this Language was already encountered bevor we add it to the current Object,
        //rather than creating a duplicate one
        if map.contains_key(&*counts.1) {
            let lang = match map.get_mut(&*counts.1) {
                None => continue,
                Some(x) => x
            };
            lang.code_count += counts.0.code_count;
            lang.empty_count += counts.0.empty_count;
            lang.comment_count += counts.0.comment_count;
        } else {
            map.insert(counts.1, counts.0);
        }
    }
    map
}

pub fn count_lines(path: PathBuf) -> Option<(LineData, &'static str)> {
    let lang = match get_lang(&path) {
        None => return None,
        Some(x) => x,
    };

    let file = match File::open(&path) {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut line_data = LineData {
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

        if l.contains(&lang.comment_symbol) {
            line_data.comment_count += 1;
            continue;
        } else if l.trim().len() == 0 {
            line_data.empty_count += 1;
        } else {
            line_data.code_count += 1;
        }
    }

    Some((line_data, lang.name))
}

fn get_lang(p: &PathBuf) -> Option<&Language> {
    let ext = p.extension()?.to_str()?;

    for lang in LANGUAGES.iter() {
        if lang.file_extension.eq(ext) {
            return Some(lang);
        }
    }
    None
}
