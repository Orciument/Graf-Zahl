use std::env;
use std::path::PathBuf;

use quicli::prelude::{CliResult};
use structopt::StructOpt;

use crate::grafzahl::count_project::analyse_project;
use crate::grafzahl::count_project::display_project;

mod grafzahl;

// pub(crate) const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";
const CONFIG_LOCATION: &str = "%LOCALAPPDATA%/graf-zahl";

pub fn get_config_location() -> String {
    CONFIG_LOCATION.replace("%LOCALAPPDATA%", &*env::var("LOCALAPPDATA").expect("Can't find Value for Env. %LOCALAPPDATA%"))
}

#[derive(StructOpt, Debug)]
struct Cli {
    // #[structopt(long = "directory", short = "d", default_value = ".")]
    #[structopt(default_value=".")]
    directory: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    display_project(
        analyse_project(
            PathBuf::from(&args.directory).canonicalize()
                .expect(&format!("Could not find an absolut File Path for Path: {}", &args.directory)))
            .unwrap()); //Currently there is no Path that can Return None

    Ok(())
}
