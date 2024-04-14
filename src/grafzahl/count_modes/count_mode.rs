use std::path::PathBuf;
use std::str::FromStr;
use colored::Colorize;
use structopt::StructOpt;
use crate::{AppState, Cli};
use crate::grafzahl::count_modes::count_mode::CountMode::*;
use crate::grafzahl::count_modes::custom_modes::char::CharCount;
use crate::grafzahl::count_modes::custom_modes::language::LanguageCount;
use crate::grafzahl::count_modes::custom_modes::line::LineCount;
use crate::grafzahl::count_modes::custom_modes::line_type::LineTypeCount;
use crate::grafzahl::count_modes::custom_modes::word::WordCount;
use crate::grafzahl::count_modes::description_printer::print_description;
use crate::grafzahl::generic_counter::generic_count;

pub fn execute_count_mode(unsafe_path: &PathBuf, state: &AppState, cli: Cli) {
    match cli.mode {
        Line => generic_count::<LineCount>(state, &cli, unsafe_path, true, true),
        Word => generic_count::<WordCount>(state, &cli, unsafe_path, true, true),
        Char => generic_count::<CharCount>(state, &cli, unsafe_path, true, true),
        LOC => generic_count::<LineTypeCount>(state, &cli, unsafe_path, true, true),
        Language => generic_count::<LanguageCount>(state, &cli, unsafe_path, true, false),
        _ => println!("This count mode is currently not supported, sorry!")
    };
}

pub fn explain_count_mode(cli: Cli) {
    match cli.mode {
        Line => print_description::<LineCount>(),
        Word => print_description::<WordCount>(),
        Char => print_description::<CharCount>(),
        LOC => print_description::<LineTypeCount>(),
        Language => print_description::<LanguageCount>(),
        _ => println!("This count mode is currently not supported, sorry!")
    };
}

#[derive(StructOpt, Debug)]
pub enum CountMode {
    Line,
    Word,
    Char,
    LOC,
    Language,
    LanguageLOC,
}

impl FromStr for CountMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_ascii_lowercase().as_str() {
            "line" => Ok(Line),
            "word" => Ok(Word),
            "w" => Ok(Word),
            "char" => Ok(Char),
            "c" => Ok(Char),
            "lines" => Ok(Line),
            "loc" => Ok(LOC),
            "lang" => Ok(Language),
            "language" => Ok(Language),
            "languageloc" => Ok(LanguageLOC),
            "langloc" => Ok(LanguageLOC),
            _ => Err(format!("{}", "NotAnOption: LINE, WORD, CHAR, LOC, LANGUAGE, LANGUAGELOC".red()))
        };
    }
}
