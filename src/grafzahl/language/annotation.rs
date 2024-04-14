use std::fmt::{Display, Formatter};
use colored::Colorize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineType {
    Comment,
    Code,
}

impl Display for LineType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self {
           LineType::Comment => write!(f, "{}", "Comm".green()),
           LineType::Code => write!(f, "CODE"),
       }
    }
}

#[derive(Clone, Debug)]
pub struct Annotation {
    pub(crate) line: String,
    pub(crate) kind: LineType,
}

impl Annotation {
    pub fn is_comment(&self) -> bool {
        self.kind == LineType::Comment
    }
}