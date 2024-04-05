use std::ops::{Add, AddAssign};
use crate::grafv3::tree_walker::FolderLoCError;
use crate::grafzahl::languages::Language;

// Tree for Folder LoC
pub(crate) struct FolderFile<'a> {
    pub(crate) name: String,
    pub(crate) ignored: bool,
    pub(crate) value: Result<FolderValue<'a>, FolderLoCError>,
}

pub(crate) enum FolderValue<'a> {
    None,
    Folder(Vec<FolderFile<'a>>),
    File(LanguageCount<'a>)
}

// Element for Top-Level Language Counting
pub(crate) struct LanguageCount<'a> {
    pub(crate) language: &'a Language,
    pub(crate) count: Count,
}

#[derive(Default, Clone, Debug)]
pub struct Count {
    pub comment_count: u32,
    pub code_count: u32,
    pub empty_count: u32,
}

impl Count {
    pub(crate) fn total(&self) -> u32 {
        self.comment_count + self.code_count + self.empty_count
    }
}

impl Add for Count {
    type Output = Count;

    fn add(self, rhs: Self) -> Self::Output {
        return Count {
            comment_count: self.comment_count + rhs.comment_count,
            code_count: self.code_count + rhs.code_count,
            empty_count: self.empty_count + rhs.empty_count,
        };
    }
}

impl AddAssign for Count {
    fn add_assign(&mut self, rhs: Self) {
        self.comment_count += rhs.comment_count;
        self.code_count += rhs.code_count;
        self.empty_count += rhs.empty_count;
    }
}
