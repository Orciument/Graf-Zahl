use std::path::PathBuf;
use std::env;
use std::process::exit;
use colored::Colorize;

pub fn get_config_path_base() -> PathBuf {
    let override_path = env::var("GRAFZAHL_CONFIG").ok();
    if override_path.is_some() {
        let path = PathBuf::from(override_path.unwrap());
        let errors = get_path_errors(&path);
        if errors.is_none() {
            return path;
        } else {
            eprintln!("{}", "ERROR: Config file Override found, but path is invalid!".red());
            eprintln!("{}", errors.unwrap());
            eprintln!("{}", path.display());
            eprintln!(" ");
            exit(2);
        }
    }
    let path = PathBuf::from(default_location());
    let errors = get_path_errors(&path);
    if errors.is_none() {
        return path;
    } else {
        eprintln!("{}", "ERROR: Default path to config files is invalid!".red());
        eprintln!("{}", errors.unwrap());
        eprintln!("{}", path.display());
        eprintln!(" ");
        exit(2);
    }
}

pub fn get_path_errors(path: &PathBuf) -> Option<String> {
    let exists = match path.try_exists() {
        Ok(v) => v,
        Err(e) => {
            return Some(format!("ERROR: Could not access path. Path malformed or missing read permissions! ioError: {}", e).red().to_string());
        }
    };
    if !exists {
        return Some("ERROR: Specified path does not exist!".red().to_string());
    }
    None
}

fn default_location() -> String {
    match env::consts::OS {
        "linux" => "/etc/opt/graf-zahl".to_string(),
        "windows" => {
            const CONFIG_LOCATION: &str = "%LOCALAPPDATA%\\graf-zahl";
            CONFIG_LOCATION.replace("%LOCALAPPDATA%", &env::var("LOCALAPPDATA").expect("Can't find Value for Env. %LOCALAPPDATA%"))
        }
        e => {
            eprintln!("{} {} {}", "Operating System:".red(), e.red(), "is not supported!".red());
            eprintln!("{}", "You can try to circumvent this error by setting a the config override env: GRAFZAHL_CONFIG to a valid path".red());
            exit(2);
        }
    }
}
