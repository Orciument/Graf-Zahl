use std::ffi::OsStr;
use std::path::PathBuf;
use colored::Colorize;
use crate::{AppState, Cli};
use crate::grafzahl::count_modes::countable::Countable;
use crate::grafzahl::io_reader::{read_dir, read_file};

#[derive(Default, Clone)]
pub(crate) struct TreeNode {
    pub(crate) string: String,
    pub(crate) members: Vec<TreeNode>,
    pub(crate) ignored: bool,
    pub(crate) errored: bool,
}

pub(crate) fn generic_count<CountMode: Countable>(state: &mut AppState, cli: &Cli, unsafe_path: &PathBuf, default_summary: bool, default_per_file: bool) {
    let path = &unsafe_path.canonicalize()
        .expect(&*format!("Absolut Path could not be found for Path: {}", unsafe_path.display()));
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", path.display());

    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap()
        .to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();

    let enable_summary = cli.summary.to_bool_or(default_summary);
    let enable_per_file = cli.per_file.to_bool_or(default_per_file);
    let count: (TreeNode, CountMode) = count_dir::<CountMode>(path, state);
    if enable_summary {
        count.1.display_summary(name);
        println!();
    }

    if enable_per_file {
        CountMode::display_legend();
        print_node(count.0, 0, cli.debug, cli.hide_errors);
    }
}

fn print_node(node: TreeNode, indent_size: usize, debug: bool, hide_errors: bool) {
    if node.errored && hide_errors { return; }
    if node.ignored && !debug { return; }
    let indent = "│  ".repeat(indent_size);
    println!("{indent}├ {}", node.string);
    for member in node.members {
        print_node(member, indent_size + 1, debug, hide_errors);
    }
}

fn count_dir<CountMode: Countable>(path: &PathBuf, state: &mut AppState) -> (TreeNode, CountMode) {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", path.display());

    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap()
        .to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();

    if state.ignore.matched(path.as_path(), path.is_dir()).is_ignore() {
        return (TreeNode {
            string: format!("{name} => [IGNORED!]"),
            ignored: true,
            ..Default::default()
        }, CountMode::default());
    }

    return if path.is_dir() {
        // Recursion
        count_folder(&path, state, &name)
    } else {
        count_file(path, &name, state)
    };
}

fn count_file<CountMode: Countable>(path: &PathBuf, name: &String, state: &mut AppState) -> (TreeNode, CountMode) {
    let ext = path.extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str().expect("Can't convert Filename into UTF-8 String!");

    let file = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            return (TreeNode {
                string: format!("{name} => [ERR] {e}").bright_red().to_string(),
                errored: true,
                ..Default::default()
            }, CountMode::default());
        }
    };
    let count = CountMode::count(file, ext, state);
    return match count {
        Ok(v) =>
            (TreeNode {
                string: format!("{name} => {v}"),
                ..Default::default()
            }, v),
        Err(e) =>
            (TreeNode {
                string: format!("{name} => [ERR] {e}").bright_red().to_string(),
                errored: true,
                ..Default::default()
            }, CountMode::default())
    };
}

fn count_folder<CountMode: Countable>(path: &PathBuf, state: &mut AppState, name: &String) -> (TreeNode, CountMode) {
    let members: Vec<(TreeNode, CountMode)> = match read_dir(path) {
        Ok(v) => v.iter().map(|p| count_dir(p, state)).collect(),
        Err(e) => {
            return (TreeNode {
                string: format!("{name}/ => [ERR] {e}").bright_red().to_string(),
                errored: true,
                ..Default::default()
            }, CountMode::default());
        }
    };
    let member_sum = members.iter().map(|x| x.1.clone()).sum();
    let member_strings = members.iter().map(|x| x.0.clone()).collect();
    return (TreeNode {
        string: format!("{name}/ => {member_sum}"),
        members: member_strings,
        ..Default::default()
    }, member_sum);
}