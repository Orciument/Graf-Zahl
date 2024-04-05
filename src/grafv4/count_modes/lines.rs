use std::iter::Sum;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::grafv4::countable::Countable;

#[derive(Default, Copy, Clone)]
pub(crate) struct LinesCount {
    pub comment_count: u32,
    pub code_count: u32,
    pub empty_count: u32,
}

impl Countable for LinesCount {
    fn count(content: Vec<String>, _: &str) -> Box<Self> {
        todo!()
    }
}

impl Add for LinesCount {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            comment_count: self.comment_count + other.comment_count,
            code_count: self.code_count + other.code_count,
            empty_count: self.empty_count + other.empty_count,
        };
    }
}

impl Sum for LinesCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(LinesCount::default(), |acc, num| acc + num)
    }
}

impl Display for LinesCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

