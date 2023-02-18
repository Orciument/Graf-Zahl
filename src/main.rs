use std::path::{PathBuf};
use crate::grafzahl::count_project::analyse_project;
use crate::grafzahl::count_project::display_project;

mod grafzahl;


fn main() {
    // let path = PathBuf::from("C:\\Users\\Master\\IdeaProjects\\Graf-Zahl");
    let path = PathBuf::from("./../JavaTwitchBot");
    let result = analyse_project(path);
    if let Some(s) = result {
        display_project(s);
    }
    display_project(analyse_project(PathBuf::from("./../BinTree")).unwrap());
    display_project(analyse_project(PathBuf::from("./../JavaTwitchBot")).unwrap());
    display_project(analyse_project(PathBuf::from("./../FraktalTree")).unwrap());
    display_project(analyse_project(PathBuf::from("./../BinTree")).unwrap());
    display_project(analyse_project(PathBuf::from("./../TwitchBot_TechTest")).unwrap());
    display_project(analyse_project(PathBuf::from("./../Schiffe_versenken_3.0")).unwrap());
    display_project(analyse_project(PathBuf::from("./../ChatServer")).unwrap());
    display_project(analyse_project(PathBuf::from("./../demo")).unwrap());
    display_project(analyse_project(PathBuf::from("./../Bot Panel Demo")).unwrap());
    display_project(analyse_project(PathBuf::from("./../clymstreamcalender")).unwrap());
    display_project(analyse_project(PathBuf::from("./../Graf-Zahl")).unwrap());
}