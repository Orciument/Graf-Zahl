use std::path::PathBuf;
use std::process::exit;
use colored::Colorize;

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use structopt::StructOpt;

use crate::{AppState, Cli};
use crate::grafzahl::config::{get_config_path_base, get_path_errors};

pub fn init_ignore_list() -> Gitignore {
    let path = get_config_path_base().join("ignore_list.gitignore");
    let errors = get_path_errors(&path);
    if errors.is_some() {
        eprintln!("{}", "ERROR: ignore file couldn't be found!".red());
        eprintln!("{}", errors.unwrap());
        eprintln!("{}", path.display());
        eprintln!(" ");
        exit(2);
    }

    let args = Cli::from_args();
    let mut builder = GitignoreBuilder::new(&args.directory);
    builder.add(path);
    let gitignore = builder.build().unwrap();
    gitignore
}

pub fn init_empty_list() -> Gitignore {
    GitignoreBuilder::new(PathBuf::new()).build().unwrap()
}

pub fn check_if_ignored(path: &PathBuf, state: &AppState) -> bool {
    state.ignore.matched(path.as_path(), path.is_dir()).is_ignore()
}
