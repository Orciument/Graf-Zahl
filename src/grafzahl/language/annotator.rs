use crate::grafzahl::language::tokeniser::rust_test::Rust;

#[derive(Debug)]
pub(crate) enum Annotation {
    Code(String),
    Comment(String),
    Empty(String),
    LineBreak,
}

pub(crate) trait Annotator {
    fn annotate(content: Vec<String>) -> Vec<Annotation>;
}

pub fn annotate(content: Vec<String>, extension: &str) -> Vec<Annotation> {
    return match extension {
        &_ => Rust::annotate(content)
    };
}