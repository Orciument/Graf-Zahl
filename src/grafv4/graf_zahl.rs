use std::ffi::OsStr;
use std::path::PathBuf;
use crate::AppState;
use crate::grafv4::countable::Countable;
use crate::grafv4::io_reader::{read_dir, read_file};
use crate::grafzahl::ignore_checker;

#[derive(Default)]
struct CountResult<CountMode: Countable> {
    value_string: String,
    value: CountMode,
    member_strings: Vec<String>,
    ignored: bool
}

pub fn count_entrypoint(path: PathBuf, state: &AppState) {
    //TODO canonicalize

    //TODO match for all CountModes, set the type parameter, and give a lambda with the right classifier

    // let count = count_path::<countModeType>(path, state);

    //TODO display
}

fn count_path<CountMode: Countable>(path: PathBuf, state: &AppState) -> CountResult<CountMode> {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());

    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap()
        .to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();

    if ignore_checker::check_if_ignored(&path, &state) {
        return CountResult {
            value_string: format!("{name} is ignored!"),
            .. Default::default()
        };
    }

    return if path.is_dir() {
        // Recursion
        count_folder(&path, state, &name)
    } else {
        count_file(path, name)
    };
}

fn count_file<CountMode: Countable>(path: PathBuf, name: String) -> CountResult<CountMode> {
    let ext = path.extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str().expect("Can't convert Filename into UTF-8 String!");

    let file = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            return CountResult {
                value_string: format!("{name} -> Err: {e}"),
                .. Default::default()
            };
        }
    };
    return CountResult {
        value_string: format!("{name} ->"),
        value: *CountMode::count(file, ext),
        .. Default::default()
    };
}

fn count_folder<CountMode: Countable>(path: &PathBuf, state: &AppState, name: &String) -> CountResult<CountMode> {
    let members: Vec<CountResult<CountMode>> = match read_dir(path) {
        Err(e) => {
            return CountResult {
                value_string: format!("{name} -> Err: {e}"),
                .. Default::default()
            };
        }
        Ok(v) => {
            v.into_iter()
                .map(|p| count_path(p, state))
                .collect()
        }
    };
    let member_sum = members.iter().map(|x| x.value.clone()).sum();
    let member_strings = members.iter().map( |x| x.value_string.clone()).collect();
    return CountResult {
        value_string: format!("{name} -> {member_sum}"),
        value: member_sum,
        member_strings,
        ignored: false,
    };
}