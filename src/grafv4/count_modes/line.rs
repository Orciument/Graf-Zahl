use std::fmt::Display;
use crate::AppState;
use crate::grafv4::countable::Countable;

pub(crate) type LineCount = u32;

impl Countable for LineCount {
    fn count(content: Vec<String>, _: &str, _: &AppState) -> Result<Self, String> {
        return Ok(content.len() as LineCount);
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {} => {} Lines", project_name, self);
    }

    fn display_legend() {}
}