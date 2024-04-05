use std::collections::HashSet;
use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use ignore::gitignore::{Gitignore};
use quicli::prelude::CliResult;
use structopt::StructOpt;
use crate::CountMode::{*};

use crate::grafzahl::ignore_checker::{init_empty_list, init_ignore_list};
use crate::grafzahl::languages::{import_languages, Language};
use crate::grafv4::graf_zahl::count_entrypoint;

mod grafzahl;
mod grafv4;

//TODO you can decuple the render of a file from the displaying of the result
// so you can just "render" the result, a error, the result, whatever; with a render function,
// save the result string and the Language Values to a result object (just 0 when error) and
// display when everything is finished.
// that way you can render as you work on the tree, and don't need to bring errors all the way up the chain,
// but can also display the result as a tree

pub fn get_config_location() -> String {
    const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";
    CONFIG_LOCATION.replace("%LOCALAPPDATA%", &env::var("LOCALAPPDATA").expect("Can't find Value for Env. %LOCALAPPDATA%"))
}

#[derive(StructOpt, Debug)]
pub enum CountMode {
    Line,
    LOC,
    Language,
    LanguageLOC,
}

impl FromStr for CountMode {
    type Err = NotAnOptionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_ascii_lowercase().as_str() {
            "line" => Ok(Line),
            "loc" => Ok(LOC),
            "lang" => Ok(Language),
            "language" => Ok(Language),
            "languageloc" => Ok(LanguageLOC),
            "langloc" => Ok(LanguageLOC),
            &_ => Err(NotAnOptionError)
        };
    }
}

impl Display for CountMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct NotAnOptionError;

impl Display for NotAnOptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(default_value = ".")]
    ///Path to the directory or File that should be counted
    directory: String,

    #[structopt(short = "d")]
    ///Enable debug mode (shows all found files and folders)
    debug: bool,

    #[structopt(short = "o")]
    /// Disable/override ignore list and search every file
    disable_ignore_list: bool,

    #[structopt(short = "c")]
    /// Show location of current config files
    show_config: bool,

    #[structopt(short = "s")]
    /// Show a Summary of all Counts for all Files
    summary: bool,
    //TODO add real summary mode, maybe return a accumulator at each step and add them all together.
    // that way it won't be a tree, but there will be the final count at the end to make the summary

    #[structopt(short = "p", long="per-file")]
    /// Show the Count for each File individually
    per_file: bool,

    #[structopt(short = "m", default_value = "loc")]
    mode: CountMode,
}

pub struct AppState {
    languages: Vec<Language>,
    ignore: Gitignore,
    missing_lang: HashSet<String>,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    if args.show_config {
        println!("The Config is located at: ");
        println!("{}", get_config_location());
        return Ok(());
    }
    let ignore = if args.disable_ignore_list {
        init_empty_list()
    } else {
        init_ignore_list()
    };
    let mut state = AppState {
        languages: import_languages(),
        ignore,
        missing_lang: HashSet::new(),
    };

    count_entrypoint(&PathBuf::from(&args.directory), &mut state, args);
    //TODO Display missing Langs
    //TODO Add command line option to hide missing Langs
    Ok(())
}
