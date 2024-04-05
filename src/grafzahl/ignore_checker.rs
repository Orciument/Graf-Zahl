use std::path::PathBuf;

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use structopt::StructOpt;

use crate::{Cli, get_config_location, AppState};

pub fn init_ignore_list() -> Gitignore {
    //TODO ignore_list should have file extension .gitignore
    let path_buff = PathBuf::from(format!("{}/ignore_list.txt", get_config_location()));
    let args = Cli::from_args();
    let mut builder = GitignoreBuilder::new(&args.directory);
    builder.add(path_buff);
    let gitignore = builder.build().unwrap();
    gitignore
}

pub fn check_if_ignored(path: &PathBuf, state: &AppState) -> bool {
    state.ignore.matched(path.as_path(), path.is_dir()).is_ignore()
}
