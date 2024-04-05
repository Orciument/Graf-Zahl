use std::ffi::OsStr;
use std::path::PathBuf;
use crate::{AppState, Cli, CountMode};
use crate::grafv4::countable::Countable;
use crate::grafv4::count_modes::line_type::LineTypeCount;
use crate::grafv4::count_modes::line::LineCount;
use crate::grafv4::io_reader::{read_dir, read_file};
use crate::grafzahl::ignore_checker;

#[derive(Default, Clone)]
pub(crate) struct TreeNode {
    pub(crate) string: String,
    pub(crate) members: Vec<TreeNode>,
    pub(crate) ignored: bool,
}

pub fn count_entrypoint(og_path: &PathBuf, state: &AppState, cli: Cli) {
    let path = &og_path.canonicalize()
        .expect(&*format!("Absolut Path could not be found for Path: {}", og_path.display()));
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", path.display());

    let count = match cli.mode {
        CountMode::Line => count_path::<LineCount>(path, state).0,
        CountMode::LOC => count_path::<LineTypeCount>(path, state).0,
        _ => TreeNode::default(),
    };

    if cli.summary {
        if path.is_dir() {
            println!("Project: {}", count.string);
        } else {
            println!("Project: {}", count.string);
        }
    }
    if cli.per_file {
        print_node(count, 0, cli.debug);
    }
}

fn print_node(node: TreeNode, indent_size: usize, debug: bool) {
    if node.ignored && !debug { return; }
    let indent = "  ".repeat(indent_size);
    println!("{indent}|- {}", node.string);
    for member in node.members {
        print_node(member, indent_size + 1, debug);
    }
}

fn count_path<CountMode: Countable>(path: &PathBuf, state: &AppState) -> (TreeNode, CountMode) {
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
        count_file(path, &name)
    };
}

fn count_file<CountMode: Countable>(path: &PathBuf, name: &String) -> (TreeNode, CountMode) {
    let ext = path.extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str().expect("Can't convert Filename into UTF-8 String!");

    let file = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            return (TreeNode {
                string: format!("{name} => [ERR] {e}"),
                ..Default::default()
            }, CountMode::default());
        }
    };
    let count = CountMode::count(file, ext);
    return (TreeNode {
        string: format!("{name} -> {count}"),
        ..Default::default()
    }, *count);
}

fn count_folder<CountMode: Countable>(path: &PathBuf, state: &AppState, name: &String) -> (TreeNode, CountMode) {
    let members: Vec<(TreeNode, CountMode)> = match read_dir(path) {
        Ok(v) => v.iter().map(|p| count_path(p, state)).collect(),
        Err(e) => {
            return (TreeNode {
                string: format!("{name}/ => [ERR] {e}"),
                ..Default::default()
            }, CountMode::default());
        }
    };
    let member_sum = members.iter().map(|x| x.1.clone()).sum();
    let member_strings = members.iter().map(|x| x.0.clone()).collect();
    return (TreeNode {
        string: format!("{name}/ => {member_sum}"),
        members: member_strings,
        ignored: false,
    }, member_sum);
}