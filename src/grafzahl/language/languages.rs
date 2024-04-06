use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use crate::{AppState, get_config_location};

#[derive(Debug, Clone)]
pub struct Language {
    pub name: String,
    pub comment_symbol: String,
    pub file_extension: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.file_extension)
    }
}

pub struct LanguageParsingError;

impl FromStr for Language {
    type Err = LanguageParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().to_string();
        let name = get_next_value(&mut s).ok_or(LanguageParsingError)?;
        let ext = get_next_value(&mut s).ok_or(LanguageParsingError)?;
        s.retain(|x| !(x == '[' || x == ']'));
        let mut vec: Vec<String> = vec![];
        while !s.is_empty() {
            vec.push(get_next_value(&mut s).ok_or(LanguageParsingError)?);
        }

        Ok(Language {
            name,
            comment_symbol: vec.first().unwrap().clone(),
            file_extension: ext,
        })
    }
}

fn get_next_value(s: &mut String) -> Option<String> {
    //Clean start "
    if s.chars().next()? == ' ' {
        s.remove(0);
    }
    if s.chars().next()? == '"' {
        s.remove(0);
    }

    //End of Value
    let offset = &s.find('\"')?;
    //Delete value from the OG String and collect the removed string as the value
    let value = s.drain(..offset).collect::<String>();
    s.remove(0); //Clean end "
    Some(value)
}

pub fn import_languages() -> Vec<Language> {
    let path: PathBuf = PathBuf::from(&format!("{}/languages.txt", get_config_location()));
    let file = match File::open(&path) {
        Ok(x) => x,
        Err(e) => panic!("Could not find Language File! Might be missing privileges, or the Path to the File may be incorrect: {e}"),
    };

    let mut languages = vec![];
    let lines = BufReader::new(file).lines();
    for l_opt in lines {
        let l = match l_opt {
            Ok(x) => x,
            Err(_) => continue,
        };
        let lang = match Language::from_str(&l) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error parsing line: ");
                eprintln!("{l}");
                continue;
            }
        };
        languages.push(lang);
    }
    languages
}

pub(crate) fn get_lang<'a>(extension: &str, state: &'a AppState) -> Result<&'a Language, String> {
    state.languages.iter()
        .find(|x| x.file_extension == extension)
        .ok_or(format!("LanguageNotFound: .{}", extension))
}
