use std::cmp::Ordering;
use crate::grafzahl::language::annotator::{Annotation, Annotator};

pub struct Rust;

impl Annotator for Rust {
    fn annotate(content: Vec<String>) -> Vec<Annotation> {
        let mut vec: Vec<Annotation> = vec![];
        let mut multi_comment_depth: i32 = 0;
        for line in content {
            let mut remaining: &str = line.as_str();

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
            } else {
                // if line has multiline comment
                let multi_line_symbols = vec![
                    Symbol { kind: SymbolKind::START, symbol: "/*".to_string() },
                    Symbol { kind: SymbolKind::END, symbol: "*/".to_string()}
                ];
                let mut prefix_len: usize = 0;
                let mut last_len: usize = 0;
                let mut prefix: &str;
                loop {
                    match get_next_symbol(&remaining[prefix_len..], &multi_line_symbols) {
                        Token::MultiStart(index, len ) => {
                            let sub = remaining.split_at(index + prefix_len);
                            println!("Start: {}|{}|{}", sub.0.split_at(prefix_len).0, sub.0.split_at(prefix_len).1, sub.1);
                            if multi_comment_depth <= 0 {
                                vec.push(Annotation::Code(String::from(sub.0)));
                            } else {
                                vec.push(Annotation::Comment(String::from(sub.0)));
                            }
                            remaining = sub.1;
                            prefix_len += len;
                            last_len = len;
                            multi_comment_depth += 1;
                        }
                        Token::MultiEnd(index, len) => {
                            let sub = remaining.split_at(index + prefix_len + 2);
                            println!("END: {}|{}|{}", sub.0.split_at(prefix_len).0, sub.0.split_at(prefix_len).1, sub.1);
                            if multi_comment_depth <= 0 {
                                vec.push(Annotation::Code(String::from(sub.0)));
                            } else {
                                vec.push(Annotation::Comment(String::from(sub.0)));
                            }
                            remaining = sub.1;
                            prefix_len -= last_len; //TODO i guess correct would be -matched char length
                            last_len = 0;
                            multi_comment_depth -= 1;
                        }
                        Token::LineEnd => {
                            println!("linend: d: {}", multi_comment_depth);
                            if multi_comment_depth <= 0 {
                                vec.push(Annotation::Code(String::from(remaining)));
                            } else {
                                vec.push(Annotation::Comment(String::from(remaining)));
                            }
                            break;
                        }
                    }
                    // if symbol.kind.eq("/*") {
                    //     let sub = remaining.split_at(symbol.index + prefix_len);
                    //     println!("Start: {}|{}|{}", sub.0.split_at(prefix_len).0, sub.0.split_at(prefix_len).1, sub.1);
                    //     if multi_comment_depth <= 0 {
                    //         vec.push(Annotation::Code(String::from(sub.0)));
                    //     } else {
                    //         vec.push(Annotation::Comment(String::from(sub.0)));
                    //     }
                    //     remaining = sub.1;
                    //     prefix_len += 2; //TODO matched char length
                    //     multi_comment_depth += 1;
                    // } else if symbol.kind.eq("*/") {
                    //     let sub = remaining.split_at(symbol.index + prefix_len + 2);
                    //     println!("END: {}|{}|{}", sub.0.split_at(prefix_len).0, sub.0.split_at(prefix_len).1, sub.1);
                    //     if multi_comment_depth <= 0 {
                    //         vec.push(Annotation::Code(String::from(sub.0)));
                    //     } else {
                    //         vec.push(Annotation::Comment(String::from(sub.0)));
                    //     }
                    //     remaining = sub.1;
                    //     prefix_len = 0; //TODO i guess correct would be -matched char length
                    //     multi_comment_depth -= 1;
                    // } else if symbol.kind.eq("LineEnd") {
                    //     println!("linend: d: {}", multi_comment_depth);
                    //     if multi_comment_depth <= 0 {
                    //         vec.push(Annotation::Code(String::from(remaining)));
                    //     } else {
                    //         vec.push(Annotation::Comment(String::from(remaining)));
                    //     }
                    //     break;
                    // }
                }
            }
        }
        vec
    }
}

struct Symbol {
    kind: SymbolKind,
    symbol: String
}

#[derive(Clone)]
enum SymbolKind {
    START,
    END,
}


struct TokenPos {
    kind: SymbolKind,
    symbol: String,
    index: usize,
}

impl Eq for TokenPos {}

impl PartialEq<Self> for TokenPos {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl PartialOrd<Self> for TokenPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for TokenPos {
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

enum Token {
    MultiStart(usize, usize),
    MultiEnd(usize, usize),
    LineEnd,
}

/// None, means that next important character is lineEnd
fn get_next_symbol(line: &str, symbols: &Vec<Symbol>) -> Token {
    let t_pos = symbols.into_iter()
        .map(|x| find_symbol(line, x))
        .filter(|x1| x1.is_some())
        .map(|x2| x2.unwrap())
        .min();
    if t_pos.is_none() {
        return Token::LineEnd;
    }
    let pos = t_pos.unwrap();
    return match pos.kind {
        SymbolKind::START => Token::MultiStart(pos.index, pos.symbol.len()),
        SymbolKind::END => Token::MultiEnd(pos.index, pos.symbol.len())
    }
}

fn find_symbol(line: &str, symbol: &Symbol) -> Option<TokenPos> {
    Some(TokenPos {
        kind: symbol.kind.clone(),
        symbol: symbol.symbol.to_string(),
        index: line.find(symbol.symbol.as_str())?,
    })
}
