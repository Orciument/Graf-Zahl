use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Add;
use crate::AppState;
use crate::grafzahl::count_modes::countable::Countable;

#[derive(Default, Clone)]
pub(crate) struct WordCount(u32);

impl Add for WordCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for WordCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, num| acc + num)
    }
}

impl Display for WordCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Countable for WordCount {
    fn count(content: Vec<String>, _: &str, _: &mut AppState) -> Result<Self, String> {
        let mut count: u32 = 0;
        for line in content {
            count += line.trim().split_whitespace().count() as u32;
        }
        return Ok(Self(count));
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {} => {} Words", project_name, self);
    }

    fn display_legend() {
        println!("Legend: => Amount of Words");
    }

    fn display_description() {
        println!("Counting Mode: Word Count");
        println!("--------------------------------------------------");
        println!("Count all Word within a File!");
        println!("Something is considered a Word when it is seperated by Unicode whitespace.");
        println!("Example: \"x = 4 + 5\" would count as 5 Words.");
    }
}