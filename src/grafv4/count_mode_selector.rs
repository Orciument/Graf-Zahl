use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use crate::{AppState, Cli};
use crate::grafv4::count_mode_selector::CountMode::*;
use crate::grafv4::count_modes::language::LanguageCount;
use crate::grafv4::count_modes::line::LineCount;
use crate::grafv4::count_modes::line_type::LineTypeCount;
use crate::grafv4::graf_zahl::generic_count;

pub fn execute_count_mode(unsafe_path: &PathBuf, state: &AppState, cli: Cli) {
    match cli.mode {
        Line => generic_count::<LineCount>(state, &cli, unsafe_path, true, true),
        LOC => generic_count::<LineTypeCount>(state, &cli, unsafe_path, true, true),
        Language => generic_count::<LanguageCount>(state, &cli, unsafe_path, true, false),
        _ => {
            println!("This count mode is currently not supported, sorry!")
        }
    };
}

#[derive(StructOpt, Debug)]
pub enum CountMode {
    Line,
    LOC,
    Language,
    LanguageLOC,
}

impl FromStr for CountMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_ascii_lowercase().as_str() {
            "line" => Ok(Line),
            "lines" => Ok(Line),
            "loc" => Ok(LOC),
            "lang" => Ok(Language),
            "language" => Ok(Language),
            "languageloc" => Ok(LanguageLOC),
            "langloc" => Ok(LanguageLOC),
            _ => Err(String::from("NotAnOption: LINE, LOC, LANGUAGE, LANGUAGELOC"))
        };
    }
}
