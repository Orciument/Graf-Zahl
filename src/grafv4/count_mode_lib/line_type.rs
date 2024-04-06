use std::iter::Sum;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::AppState;
use crate::grafv4::countable::Countable;
use crate::grafzahl::languages::{get_lang};

#[derive(Default, Copy, Clone)]
pub(crate) struct LineTypeCount {
    pub comment_count: u32,
    pub code_count: u32,
    pub empty_count: u32,
}

impl Countable for LineTypeCount {
    fn count(content: Vec<String>, extension: &str, state: &AppState) -> Result<Self, String> {
        let lang = get_lang(extension, state)?;

        let mut line_data: LineTypeCount = Default::default();
        for l in content {
            if l.contains(&lang.comment_symbol) {
                line_data.comment_count += 1;
            } else if l.trim().is_empty() {
                line_data.empty_count += 1;
            } else {
                line_data.code_count += 1;
            }
        }
        return Ok(line_data);
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {}:", project_name);
        println!("  Lines of Code:      {}", self.code_count);
        println!("  Lines of Comments:  {}", self.comment_count);
        println!("  Lines of New Lines: {}", self.empty_count);
        println!("Total: {}", self.code_count + self.comment_count + self.empty_count);
    }

    fn display_legend() {
        println!("Legend: => (Lines of Code, Lines with Comments, New Lines)");
    }
}

impl Add for LineTypeCount {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            comment_count: self.comment_count + other.comment_count,
            code_count: self.code_count + other.code_count,
            empty_count: self.empty_count + other.empty_count,
        };
    }
}

impl Sum for LineTypeCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, num| acc + num)
    }
}

impl Display for LineTypeCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.code_count, self.comment_count, self.empty_count)
    }
}

