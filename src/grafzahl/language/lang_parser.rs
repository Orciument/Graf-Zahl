use colored::Colorize;
use regex::Regex;
use structopt::lazy_static::lazy_static;
use crate::grafzahl::language::languages::Language;

lazy_static! {
    static ref REGEX: Regex = Regex::new("^ *\"(.*)\" *, *\"(.*)?\" *, *\\[(.*)] *, *\\[(.*)] *, *\\[(.*)] *$").unwrap();
}


pub fn parse_langs(content: Vec<String>) -> Vec<Language> {
    let mut vec = vec![];
    for line in content {
        if line.trim().is_empty() {
            continue;
        } else if line.trim().starts_with("#") {
            continue;
        }
        match parse_line(&line) {
            Ok(v) => vec.push(v),
            Err(_) => {
                eprintln!("{}", "Couldn't parse line Language Definition is malformed!".red().underline());
                eprintln!("{}", line.red());
                eprintln!(" ");
            }
        }
    }
    vec
}

/// Parse a language File line using Regex capturing groups, whitespace is not significant. A Unit Error is returned when the Regex did not match the given input line
fn parse_line(line: &String) -> Result<Language, ()> {
    let test_line = r#" "rs", "Rust", [ "//", "///"], ["/*"], ["*/"]"#;
    let caps = REGEX.captures(line).ok_or(())?;
    // these shouldn't be able to fail because all our match groups are mandatory
    // and the capturing itself should fail if they aren't provided by the user
    let extension = caps.get(1).unwrap().as_str();
    let name = caps.get(2).unwrap().as_str();
    let inline = parse_array(caps.get(3).unwrap().as_str());
    let block_start = parse_array(caps.get(4).unwrap().as_str());
    let block_end = parse_array(caps.get(5).unwrap().as_str());
    Ok(Language {
        name: name.to_string(),
        file_extension: extension.to_string(),
        inline_symbols: inline,
        block_start_symbols: block_start,
        block_end_symbols: block_end,
    })
}

/// Splits the raw Array content by its element divider: , <br>
/// This means it is not possible to use , in a symbol because it will lead to parsing errors
fn parse_array(raw: &str) -> Vec<String> {
    raw.split(",")
        .map(|x| x.trim())
        .filter(|x2| {
            if x2.len() == 1 {
                eprintln!("{}: A Symbol Definition has a unclosed String!\n", "warning".bright_yellow().bold());
            }
            x2.len() >= 2
        })
        .map(|x1| x1[1..x1.len() - 1].replace("\\\"", "\""))
        .collect()
}