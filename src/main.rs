use std::env;
use std::path::PathBuf;

use quicli::prelude::CliResult;
use structopt::StructOpt;

use crate::grafzahl::print_project::count_from_path;

mod grafzahl;

// pub(crate) const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";
const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";

pub fn get_config_location() -> String {
    CONFIG_LOCATION.replace("%LOCALAPPDATA%", &env::var("LOCALAPPDATA").expect("Can't find Value for Env. %LOCALAPPDATA%"))
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
    count_from_path(PathBuf::from(&args.directory));
    Ok(())
}
