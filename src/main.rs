use std::collections::HashSet;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use colored::Colorize;

use ignore::gitignore::Gitignore;
use quicli::prelude::CliResult;
use structopt::StructOpt;
use grafzahl::config;

use crate::grafzahl::ignore_checker::{init_empty_list, init_ignore_list};
use crate::grafzahl::language::languages::{import_languages, Language, print_missing};
use crate::grafzahl::count_modes::count_mode::{CountMode, execute_count_mode, explain_count_mode};

mod grafzahl;

#[derive(StructOpt, Debug)]
pub(crate) enum Override {
    Enable,
    Disable,
    None,
}

impl Override {
    pub(crate) fn to_bool_or(&self, default: bool) -> bool {
        return match self {
            Override::Enable => true,
            Override::Disable => false,
            Override::None => default,
        };
    }
}

impl FromStr for Override {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_ascii_lowercase().as_str() {
            "on" => Ok(Override::Enable),
            "enable" => Ok(Override::Enable),
            "off" => Ok(Override::Disable),
            "disable" => Ok(Override::Disable),
            "none" => Ok(Override::None),
            "default" => Ok(Override::None),
            _ => Err(format!("{}", "NotAnOption: ON, OFF".red()))
        };
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

    #[structopt(short = "s", long = "silent")]
    /// Hide all errors that occur
    hide_errors: bool,

    #[structopt(short = "c")]
    /// Show location of current config files
    show_config: bool,

    #[structopt(short = "e", long = "explain")]
    /// Explain selected count mode
    explain_mode: bool,

    #[structopt(short = "u", default_value = "none")]
    /// Show a Summary of all Counts for all Files
    summary: Override,

    #[structopt(short = "p", default_value = "none", long = "per-file")]
    /// Show the Count for each File individually
    per_file: Override,

    #[structopt(short = "m", default_value = "loc", long = "mode")]
    /// How file are to be counted
    mode: CountMode,
}

pub struct AppState {
    languages: Vec<Language>,
    ignore: Gitignore,
    missing_lang: HashSet<String>,
}


fn main() -> CliResult {
    let cli = Cli::from_args();
    if cli.show_config {
        println!("The Config is currently located at: ");
        println!("{}", config::get_config_path_base().display());
        return Ok(());
    } else if cli.explain_mode {
        explain_count_mode(cli);
        return Ok(());
    }

    let unsafe_path = &PathBuf::from(&cli.directory);
    let safe_path = match unsafe_path.canonicalize() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Absolut Path could not be found for Path: {}", unsafe_path.display());
            eprintln!("{e}");
            exit(74);
        }
    };

    let ignore = if cli.disable_ignore_list {
        init_empty_list()
    } else {
        init_ignore_list()
    };

    let mut state = AppState {
        languages: import_languages(),
        ignore,
        missing_lang: HashSet::new(),
    };

    let hide_errors= cli.hide_errors;
    execute_count_mode(&safe_path, &mut state, cli);

    if !hide_errors {
        println!();
        print_missing(state.missing_lang);
    }
    Ok(())
}
