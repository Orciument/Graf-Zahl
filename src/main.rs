use std::collections::{HashSet};
use std::env;
use std::path::PathBuf;
use ignore::gitignore::Gitignore;

use quicli::prelude::CliResult;
use structopt::StructOpt;
use crate::grafzahl::ignore_checker::init_ignore_list;
use crate::grafzahl::languages::{import_languages, Language};
use crate::grafzahl::print_project::count_from_path;

mod grafzahl;

pub fn get_config_location() -> String {
    const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";
    CONFIG_LOCATION.replace("%LOCALAPPDATA%", &env::var("LOCALAPPDATA").expect("Can't find Value for Env. %LOCALAPPDATA%"))
}

pub struct State {
    languages: Vec<Language>,
    ignore: Gitignore,
    missing_lang: HashSet<String>
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(default_value = ".")]
    ///Path to the directory or File that should be counted
    directory: String,

    #[structopt(short = "d")]
    ///Enable debug mode (shows all found files and folders)
    debug: bool,

    #[structopt(short = "f")]
    ///Shows the LOC per Folder instead of a toplevel Language overview
    per_folder: bool,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    let mut state = State {
        languages: import_languages(),
        ignore: init_ignore_list(),
        missing_lang: HashSet::new(),
    };
    count_from_path(PathBuf::from(&args.directory), &mut state);

    //TODO Display missing Langs
    //TODO Add command line option to hide these
    Ok(())
}
