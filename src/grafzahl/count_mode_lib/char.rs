use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Add;
use crate::AppState;
use crate::grafzahl::countable::Countable;

#[derive(Default, Clone)]
pub(crate) struct CharCount(u32);

impl Add for CharCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for CharCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, num| acc + num)
    }
}

impl Display for CharCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Countable for CharCount {
    fn count(content: Vec<String>, _: &str, _: &AppState) -> Result<Self, String> {
        let mut count: u32 = 0;
        for line in content {
            count += line.len() as u32;
        }
        return Ok(Self(count));
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {} => {} Character", project_name, self);
    }

    fn display_legend() {
        println!("Legend: => Amount of Chars")
    }

    fn display_description() {
        println!("Counting Mode: Character Count");
        println!("--------------------------------------------------");
        println!("Count all Characters within a File!");
    }
}