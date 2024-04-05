use std::collections::HashSet;
use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use ignore::gitignore::Gitignore;
use quicli::prelude::CliResult;
use structopt::StructOpt;
use crate::CountMode::{Line, Word};
use crate::grafv2::indexer::index_directory;
use crate::grafv2::printer::{print_language_entry, print_structure_entry};

use crate::grafzahl::ignore_checker::init_ignore_list;
use crate::grafzahl::languages::{import_languages, Language};
use crate::grafzahl::print_project::count_from_path;

mod grafzahl;
mod grafv2;
// mod grafv3;
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
    Word,
    Char,
}
impl FromStr for CountMode {
    type Err = NotAnOptionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_ascii_lowercase().as_str() {
            "line" => Ok(Line),
            "word" => Ok(Word),
            "char" => Ok(Word),
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
impl Display for  NotAnOptionError {
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

    #[structopt(short = "s")]
    ///Shows the LOC per Folder instead of a toplevel Language overview
    per_folder: bool,

    #[structopt(short = "c")]
    /// Show location of current config files
    show_config: bool,

    #[structopt(short = "o")]
    /// Disable/override ignore list and search every file
    disable_ignore_list: bool,

    #[structopt(short = "n")]
    new: bool,
}

pub struct AppState {
    languages: Vec<Language>,
    ignore: Gitignore,
    count_mode: CountMode,
    missing_lang: HashSet<String>,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    if args.show_config {
        println!("The Config is located at: ");
        println!("{}", get_config_location());
        return Ok(())
    }

    let mut state = AppState {
        languages: import_languages(),
        ignore: init_ignore_list(),
        count_mode: Line,
        missing_lang: HashSet::new(),
    };

    if args.new {
        let a = index_directory(&PathBuf::from(&args.directory), args.disable_ignore_list, &mut state);
        if args.per_folder {
            print_structure_entry(a, args.disable_ignore_list);
        } else {
            print_language_entry(a);
        }
    }
    else {
        count_from_path(PathBuf::from(&args.directory), &mut state);
    }
    //TODO Display missing Langs
    //TODO Add command line option to hide these
    Ok(())
}
