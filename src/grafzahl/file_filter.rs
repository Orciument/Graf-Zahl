use std::path::PathBuf;

pub struct Language {
    pub name: &'static str,
    pub comment_symbol: &'static str,
    pub file_extension: &'static str,
}

pub fn filter_files(vec: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut filtered: Vec<PathBuf> = vec![];

    for p in vec {
        if file_is_supported(p) {
            filtered.push(p.clone());
        }
    }
    filtered
}

fn file_is_supported(p: &PathBuf) -> bool {
    let ext = match p.extension() {
        None => return false,
        Some(x) => match x.to_str() {
            None => return false,
            Some(y) => y
        }
    };
    for lang in supported_languages() {
        if lang.file_extension.eq(ext) {
            return true;
        }
    }
    false
}

pub fn get_language_info(p: &PathBuf) -> Option<Language> {
    let ext = p.extension()?.to_str()?;

    for lang in supported_languages() {
        if lang.file_extension.eq(ext) {
            return Some(lang);
        }
    }
    None
}

fn supported_languages() -> Vec<Language> {
    let file_types: Vec<Language> = vec![Language {
        name: "Rust",
        comment_symbol: "//",
        file_extension: "rs",
    }, Language {
        name: "Java",
        comment_symbol: "//",
        file_extension: "java",
    }, Language {
        name: "",
        comment_symbol: "",
        file_extension: "",
    }
    ];
    file_types
}