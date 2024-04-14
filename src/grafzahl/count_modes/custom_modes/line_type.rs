use std::iter::Sum;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::AppState;
use crate::grafzahl::count_modes::countable::Countable;
use crate::grafzahl::language::annotator::annotate;
use crate::grafzahl::language::languages::{get_lang};

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
        let annotated = annotate(content, &lang.comment_symbol, &vec![], &vec![]);
        for l in annotated {
            if l.is_comment() {
                line_data.comment_count += 1;
            } else if l.line.trim().is_empty() {
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

    fn display_description() {
        println!("Counting Mode: Line Type Count");
        println!("--------------------------------------------------");
        println!("Classifies the lines into 3 Categories, code, comments, and empty lines!");
        println!("A line is empty when it is empty after all whitespace has been trimmed away (the line ending character is also ignored).");
        println!("A line is considered a Comment when it contains a String that starts or stops a Comment in the language associated with the file Extension.");
        println!("Everything else is considered a line of Code.");
        println!();
        println!("Languages and what Characters start a comment can be configured in the language config file.");
        println!("The location of the config files can be found by using '-c' or '--config'.");
        println!("The Syntax for the config is:");
        println!("\"LANGUAGE_NAME\" \"FILE_EXTENSION\" [\"COMMENT_STRING\" \"COMMENT_STRING\"]");
        println!("Example: \"Rust\" \"rs\" [\"//\" \"///\"]");
        println!("You can define as many comment strings as you like");
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

