use std::ffi::OsStr;
use std::path::PathBuf;
use crate::AppState;
use crate::grafv4::countable::Countable;
use crate::grafv4::io_reader::{read_dir, read_file};
use crate::grafzahl::ignore_checker;

#[derive(Default, Clone)]
struct TreeNode {
    string: String,
    members: Vec<TreeNode>,
    ignored: bool,
}

pub fn count_entrypoint(path: PathBuf, state: &AppState) {
    //TODO canonicalize

    //TODO match for all CountModes, set the type parameter, and give a lambda with the right classifier

    // let count = count_path::<countModeType>(path, state).0;

    //TODO display
}

fn count_path<CountMode: Countable>(path: PathBuf, state: &AppState) -> (TreeNode, CountMode) {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());

    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap()
        .to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();

    if ignore_checker::check_if_ignored(&path, &state) {
        return (TreeNode {
            string: format!("{name} is ignored!"),
            ignored: true,
            ..Default::default()
        }, CountMode::default());
    }

    return if path.is_dir() {
        // Recursion
        count_folder(&path, state, &name)
    } else {
        count_file(path, name)
    };
}

fn count_file<CountMode: Countable>(path: PathBuf, name: String) -> (TreeNode, CountMode) {
    let ext = path.extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str().expect("Can't convert Filename into UTF-8 String!");

    let file = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            return (TreeNode {
                string: format!("{name} -> Err: {e}"),
                ..Default::default()
            }, CountMode::default());
        }
    };
    return (TreeNode {
        string: format!("{name} ->"),
        ..Default::default()
    }, *CountMode::count(file, ext));
}

fn count_folder<CountMode: Countable>(path: &PathBuf, state: &AppState, name: &String) -> (TreeNode, CountMode) {
    let members: Vec<(TreeNode, CountMode)> = match read_dir(path) {
        Err(e) => {
            return (TreeNode {
                string: format!("{name} -> Err: {e}"),
                ..Default::default()
            }, CountMode::default());
        }
        Ok(v) => {
            v.into_iter()
                .map(|p| count_path(p, state))
                .collect()
        }
    };
    let member_sum = members.iter().map(|x| x.1.clone()).sum();
    let member_strings = members.iter().map(|x| x.0.clone()).collect();
    return (TreeNode {
        string: format!("{name} -> {member_sum}"),
        members: member_strings,
        ignored: false,
    }, member_sum);
}