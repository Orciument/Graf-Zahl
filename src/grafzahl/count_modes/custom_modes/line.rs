use crate::AppState;
use crate::grafzahl::count_modes::countable::Countable;

pub(crate) type LineCount = u32;

impl Countable for LineCount {
    fn count(content: Vec<String>, _: &str, _: &AppState) -> Result<Self, String> {
        return Ok(content.len() as LineCount);
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {} => {} Lines", project_name, self);
    }

    fn display_legend() {}

    fn display_description() {
        println!("Counting Mode: Line Count");
        println!("--------------------------------------------------");
        println!("Count all Lines within a File!");
    }
}