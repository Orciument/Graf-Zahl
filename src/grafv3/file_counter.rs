use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::grafv3::tree::{Count, LanguageCount};
use crate::grafv3::tree_walker::FolderLoCError::EncodingNotSupported;
use crate::grafzahl::languages::Language;

pub(crate) fn count_file(file: File, language: &Language) -> Result<LanguageCount, EncodingNotSupported> {

    let mut count: Count = Default::default();

    let lines = BufReader::new(file).lines();
    for line_result in lines {
        let l = &line_result.or_else(|_| { Err(EncodingNotSupported) })?;

        if l.contains(&language.comment_symbol) {
            count.comment_count += 1;
        } else if l.trim().is_empty() {
            count.empty_count += 1;
        } else {
            count.code_count += 1;
        }
    }

    Ok(LanguageCount {
        language,
        count,
    })
}