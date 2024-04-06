use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use crate::{AppState, Cli};
use crate::grafv4::count_mode::CountMode::*;
use crate::grafv4::count_mode_lib::char::CharCount;
use crate::grafv4::count_mode_lib::language::LanguageCount;
use crate::grafv4::count_mode_lib::line::LineCount;
use crate::grafv4::count_mode_lib::line_type::LineTypeCount;
use crate::grafv4::count_mode_lib::word::WordCount;
use crate::grafv4::description_printer::print_description;
use crate::grafv4::graf_zahl::generic_count;

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
            _ => Err(String::from("NotAnOption: LINE, WORD, CHAR, LOC, LANGUAGE, LANGUAGELOC"))
        };
    }
}
