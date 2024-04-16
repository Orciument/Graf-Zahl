use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use crate::AppState;

pub(crate) trait Countable: Add + Sum + Display + Default + Clone {
    fn count(content: Vec<String>, extension: &str, state: &mut AppState) -> Result<Self, String>;

    fn display_summary(self, project_name: String);

    fn display_legend();

    fn display_description();
}