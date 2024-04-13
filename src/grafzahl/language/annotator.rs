use std::mem::{discriminant};
use std::ops::Add;
use crate::grafzahl::language::annotator::Annotation::{Code, Comment, Empty, LineBreak};
use crate::grafzahl::language::tokeniser::rust_test::Rust;

#[derive(Debug, Clone)]
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

pub fn collapse(annotated: Vec<Annotation>) -> Vec<Annotation> {
    // if linebrack copy and continue
    // see if it is the same as the last one
    //  get string from it
    //  combine with stored string
    // if not
    //  push stored one
    //  store current one
    if annotated.len() <= 1 { return annotated; }
    let mut new = vec![];
    let mut last = &annotated[0];
    let mut last_string: String = get_string(&annotated[0]);
    for i in 1..annotated.len() {
        let current = &annotated[i];
        if discriminant(current) == discriminant(&LineBreak) {
            new.push(create_from_store(last, last_string.clone()));
            last_string = String::new();
            last = current;
            new.push(LineBreak);
            continue;
        } else if discriminant(current) == discriminant(last) { // check if from the same type
            last_string = last_string.add(get_string(current).as_str());
            continue;
        } else {
            new.push(create_from_store(last, last_string));
            last_string = get_string(current);
            last = current;
        }
    }
    if !last_string.is_empty() {
        new.push(create_from_store(last, last_string));
    }
    new
}

fn create_from_store(last_type: &Annotation, stored: String) -> Annotation {
    return match last_type {
        Code(_) => Code(stored),
        Comment(_) => Comment(stored),
        Empty(_) => Empty(stored),
        LineBreak => LineBreak,
    }
}

fn get_string(annotation: &Annotation) -> String {
    return match annotation {
        Code(s) => s,
        Comment(s) => s,
        Empty(s) => s,
        LineBreak => panic!("Cant get String from LineBreak!")
    }.clone();
}