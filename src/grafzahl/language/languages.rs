use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use colored::Colorize;
use crate::{AppState, get_config_location};
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

pub fn import_languages() -> Result<Vec<Language>, String> {
    let path: PathBuf = PathBuf::from(&format!("{}/languages.txt", get_config_location()));
    let exists = path.try_exists().or(Err("ERROR: Could not access Language Config File. Path malformed or missing read permissions!".bright_red().to_string()))?;
    if !exists {
        return Err("ERROR: Specified Config File location does not exist!".red().to_string());
    }
    let lines = read_file(&path).or_else(|e| Err(e.to_string()))?;
    return Ok(parse_langs(lines));
}

pub(crate) fn get_lang<'a>(extension: &str, state: &'a AppState) -> Result<&'a Language, String> {
    state.languages.iter()
        .find(|x| x.file_extension == extension)
        .ok_or(format!("LanguageNotFound: .{}", extension))
}
