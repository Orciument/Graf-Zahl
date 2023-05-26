use std::env;
use std::path::PathBuf;

use quicli::prelude::CliResult;
use structopt::StructOpt;

use crate::grafzahl::count_project::analyse_project;
use crate::grafzahl::count_project::display_project;
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

    #[structopt(long = "tree_indexer", short = "t")]
    ///Enable beta tree_indexer (allows count per directory, with -f)
    tree_indexer: bool,

    #[structopt(short = "d")]
    ///Enable debug mode (shows all found files and folders)
    debug: bool,

    #[structopt(short="f")]
    ///Shows the LOC per Folder instead of a toplevel Language overview
    per_folder: bool,
}


fn main() -> CliResult {
    let args = Cli::from_args();
    if args.tree_indexer {
        count_from_path(PathBuf::from(&args.directory))
    } else {
        display_project(
            analyse_project(
                PathBuf::from(&args.directory).canonicalize()
                    .unwrap_or_else(|_| panic!("Could not find an absolut File Path for Path: {}", &args.directory)))
                .unwrap()); //Currently there is no Path that can Return None
    }
    Ok(())
}
