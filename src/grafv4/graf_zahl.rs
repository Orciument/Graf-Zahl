use std::ffi::OsStr;
use std::iter::Sum;
use std::path::PathBuf;
use crate::AppState;
use crate::grafv4::countable::Countable;
use crate::grafv4::io_reader::{read_dir, read_file};
use crate::grafzahl::ignore_checker;

struct CountResult<CountMode: Sum> {
    result_value: String,
    //TODO result as value, so that the levels on top can calc total
    value: CountMode,
    members: Option<Vec<CountResult<CountMode>>>,
}

pub fn count_entrypoint(path: PathBuf, state: &AppState) {
    //TODO canonicalize

    //TODO match for all CountModes, set the type parameter, and give a lambda with the right classifier

    // let count = count_path::<countModeType>(path, state);

    //TODO display
}

fn count_path<CountMode: Countable>(path: PathBuf, state: &AppState) -> Option<CountResult<CountMode>> {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", &path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", &path.display());

    // This unwrap is safe, because all "/.." are resolved when we canonicalize the Path
    let name = path.file_name().unwrap()
        .to_str().expect("Unable to convert File/Folder Name into Unicode!").to_string();

    if ignore_checker::check_if_ignored(&path, &state) {
        // if !state.debug {
        //     return None;
        // }

        // if path.is_dir() {
        //     let ext = path.extension()
        //         .unwrap_or_else(|| OsStr::new(""))
        //         .to_str().expect("Can't convert Filename into UTF-8 String!");
        //     return Some(CountResult {
        //         result_value: format!("{name}.{ext} is ignored!"),
        //         value: CountMode::default(),
        //         members: None,
        //     })
        // }
        // return Some(CountResult {
        //     result_value: format!("{name} is ignored!"),
        //     value: CountMode::default(),
        //     members: None,
        // })
        return None;
    }


    return if path.is_dir() {
        // Recursion
        Some(count_folder(&path, state, &name))
    } else {
        Some(count_file(path, name))
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
                result_value: format!("{name}.{ext} -> Err: {e}"),
                value: CountMode::default(),
                members: None,
            };
        }
    };
    return CountResult {
        result_value: format!("{name}.{ext} ->"),
        value: *CountMode::count(file),
        members: None,
    };
}

fn count_folder<CountMode: Countable>(path: &PathBuf, state: &AppState, name: &String) -> CountResult<CountMode> {
    let vec: Vec<CountResult<CountMode>> = match read_dir(path) {
        Err(e) => {
            return CountResult {
                result_value: format!("{name} -> Err: {e}"),
                value: CountMode::default(),
                members: None,
            };
        }
        Ok(v) => {
            v.into_iter()
                .map(|p| count_path(p, state))
                .filter(|o| o.is_some())
                .map(|oo| oo.unwrap())
                .collect()
        }
    };
    let sum = vec.iter().map(|x| x.value.clone()).sum();
    return CountResult {
        result_value: format!("{name} -> {sum}"),
        value: sum,
        members: Some(vec),
    };
}