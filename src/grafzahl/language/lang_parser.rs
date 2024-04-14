use colored::Colorize;
use regex::Regex;
use crate::grafzahl::language::languages::Language;

fn parse_langs(content: Vec<String>) -> Vec<Language> {
    let mut vec = vec![];
    for line in content {
        match parse_line(&line) {
            Ok(v) => vec.push(v),
            Err(v) => {
                eprintln!("{}", "Error parsing line:".red().underline());
                eprintln!("{}", v.red());
                eprintln!(" ");
            },
        }
    }
    vec
}

/// Parse a language File line using Regex capturing groups, whitespace is not significant
pub fn parse_line(line: &String) -> Result<Language, String> {
    let r = Regex::new("^ *\"(.*)\" *, *\"(.*)?\" *, *\\[(.*)] *, *\\[(.*)] *, *\\[(.*)] *$").unwrap();
    let test_line = r#" "rs", "Rust", [ "//", "///"], ["/*"], ["*/"]"#;
    let caps = r.captures(test_line).ok_or("Language Definition is malformed!".red().to_string())?;
    // these shouldn't be able to fail because all our match groups are mandatory
    // and the capturing itself should fail if they aren't provided by the user
    let extension = caps.get(1).unwrap().as_str();
    let name = caps.get(2).unwrap().as_str();
    let inline = parse_array(caps.get(3).unwrap().as_str());
    let block_start = parse_array(caps.get(4).unwrap().as_str());
    let block_end = parse_array(caps.get(5).unwrap().as_str());
    println!("{}", extension);
    println!("{}", name);
    println!("{:?}", inline);
    println!("{:?}", block_start);
    println!("{:?}", block_end);
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
        .map(|x1| x1[1..x1.len() - 1].to_string())
        .collect()
}