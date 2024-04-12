use std::cmp::Ordering;
use crate::grafzahl::language::annotator::{Annotation, Annotator};

pub struct Rust;

impl Annotator for Rust {
    fn annotate(content: Vec<String>) -> Vec<Annotation> {
        let mut vec: Vec<Annotation> = vec![];
        let mut multi_comment_depth: i32 = 0;
        for line in content {
            // early return if line has line comment
            let single_comment = next_symbol_pos(&line, vec!["//", "///"]);
            if single_comment.is_some() {
                if multi_comment_depth <= 0 {
                    let s = line.split_at(single_comment.unwrap());
                    vec.push(Annotation::Code(String::from(s.0)));
                    vec.push(Annotation::Comment(String::from(s.1)));
                } else {
                    vec.push(Annotation::Comment(line));
                }
                continue;
            }
            // if line has multiline comment
            let multi_line_symbols = vec![
                BlockSymbols { start: true, symbol: "/*".to_string() },
                BlockSymbols { start: false, symbol: "*/".to_string() },
            ];
            parse_multi_comments(&mut vec, multi_comment_depth, line.as_str(), &multi_line_symbols);
        }
        vec
    }
}

/// consumes all Block Comment Symbols until the next linebreak, ignores line comments
fn parse_multi_comments(vec: &mut Vec<Annotation>, mut multi_comment_depth: i32, line: &str, multi_line_symbols: &Vec<BlockSymbols>) {
    let mut remaining: &str = line;
    let mut prefix: &str = "";
    loop {
        match get_next_symbol(&remaining, &multi_line_symbols) {
            Token::MultiStart(index, len) => {
                let sub = remaining.split_at(index);
                push_string(vec, multi_comment_depth, prefix, sub.0);
                prefix = &sub.1[..len];
                remaining = &sub.1[len..];
                multi_comment_depth += 1;
            }
            Token::MultiEnd(index, len) => {
                let sub = remaining.split_at(index + len);
                push_string(vec, multi_comment_depth, prefix, sub.0);
                prefix = "";
                remaining = sub.1;
                multi_comment_depth -= 1;
            }
            Token::LineEnd => {
                push_string(vec, multi_comment_depth, prefix, remaining);
                break;
            }
        }
    }
}

fn push_string(mut vec: &mut Vec<Annotation>, multi_comment_depth: i32, prefix: &str, remaining: &str) {
    if multi_comment_depth <= 0 {
        vec.push(Annotation::Code(String::from(prefix.to_owned() + remaining)));
    } else {
        vec.push(Annotation::Comment(String::from(prefix.to_owned() + remaining)));
    }
}
/// Multiline comment Symbol
struct BlockSymbols {
    start: bool,
    symbol: String,
}

#[derive(Eq, PartialEq)]
struct BlockSymbolPos {
    index: usize,
    start: bool,
    symbol: String,
}

impl PartialOrd<Self> for BlockSymbolPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for BlockSymbolPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

fn next_symbol_pos(line: &String, symbols: Vec<&str>) -> Option<usize> {
    symbols.iter()
        .map(|x| line.find(x))
        .filter(|x1| x1.is_some())
        .map(|x2| x2.unwrap())
        .min()
}

#[derive(Eq, PartialEq)]
enum Token {
    MultiStart(usize, usize),
    MultiEnd(usize, usize),
    LineEnd,
}

/// None, means that next important character is lineEnd
fn get_next_symbol(line: &str, symbols: &Vec<BlockSymbols>) -> Token {
    let t_pos = symbols.into_iter()
        .map(|x| find_symbol(line, x))
        .filter(|x1| x1.is_some())
        .map(|x2| x2.unwrap())
        .min();
    if t_pos.is_none() {
        return Token::LineEnd;
    }
    let pos = t_pos.unwrap();
    return match pos.start {
        true => Token::MultiStart(pos.index, pos.symbol.len()),
        false => Token::MultiEnd(pos.index, pos.symbol.len())
    };
}

fn find_symbol(line: &str, symbol: &BlockSymbols) -> Option<BlockSymbolPos> {
    Some(BlockSymbolPos {
        index: line.find(symbol.symbol.as_str())?,
        start: symbol.start,
        symbol: symbol.symbol.to_string(),
    })
}
