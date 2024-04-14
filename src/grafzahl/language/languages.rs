use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::exit;
use colored::Colorize;
use crate::AppState;
use crate::grafzahl::config::{get_config_path_base, get_path_errors};
use crate::grafzahl::io_reader::{read_file};
use crate::grafzahl::language::lang_parser::parse_langs;

#[derive(Debug, Clone)]
pub struct Language {
    pub name: String,
    pub file_extension: String,
    pub inline_symbols: Vec<String>,
    pub block_start_symbols: Vec<String>,
    pub block_end_symbols: Vec<String>,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.file_extension)
    }
}

pub fn import_languages() -> Vec<Language> {
    let path: PathBuf = get_config_path_base().join("languages.txt");
    let errors = get_path_errors(&path);
    if errors.is_some() {
        eprintln!("{}", "ERROR: Langauge config file couldn't be found!".red());
        eprintln!("{}", errors.unwrap());
        eprintln!("{}", path.display());
        eprintln!(" ");
        exit(2);
    }

    let lines = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(2)
        }
    };
    return parse_langs(lines);
}

pub(crate) fn get_lang<'a>(extension: &str, state: &'a AppState) -> Result<&'a Language, String> {
    state.languages.iter()
        .find(|x| x.file_extension == extension)
        .ok_or(format!("LanguageNotFound: .{}", extension))
}
