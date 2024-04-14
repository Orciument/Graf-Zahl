use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineType {
    Comment,
    Code,
}

impl Display for LineType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineType::Comment => write!(f, "{}", "Comm"),
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

/// Annotates Lines with if they are a comment or not
pub fn annotate(lines: Vec<String>, inline_symbols: &Vec<String>, block_start: &Vec<String>, block_end: &Vec<String>) -> Vec<Annotation> {
    let mut ann = vec![];
    let mut comment_depth: i32 = 0;
    for line in lines {
        let inline = count_occurrences(&line, &inline_symbols);
        let start = count_occurrences(&line, &block_start);
        let end = count_occurrences(&line, &block_end);
        comment_depth += start;
        if inline > 0 {
            ann.push(Annotation { line, kind: LineType::Comment });
        } else if comment_depth > 0 {
            ann.push(Annotation {line, kind: LineType::Comment});
        } else {
            ann.push(Annotation {line, kind: LineType::Code});
        }
        comment_depth -= end;
    }
    ann
}

fn count_occurrences(line: &String, find: &Vec<String>) -> i32 {
    let count: usize = find.iter()
        .map(|x| line.matches(x).count())
        .sum();
    count as i32
}