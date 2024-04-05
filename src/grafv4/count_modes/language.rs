use std::iter::Sum;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::grafv4::countable::Countable;

#[derive(Default, Copy, Clone)]
pub(crate) struct LanguageCount {}

impl Add for LanguageCount {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sum for LanguageCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        todo!()
    }
}

impl Display for LanguageCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Countable for LanguageCount {
    fn count(content: Vec<String>, extension: &str) -> Box<Self> {
        todo!()
    }
}

