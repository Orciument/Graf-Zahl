use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::ops::{Add, AddAssign};

use crate::grafv2::tree::CountFileError::IoError;

pub(crate) trait LocTotal {
    fn total(self) -> u32;
    fn total_types(self) -> Count;
    fn total_langs(self) -> Vec<LanguageCount>;
}

pub(crate) struct DirectoryMember {
    pub(crate) name: String,
    pub(crate) ignored: bool,
    pub(crate) member: FolderElement,
}

pub(crate) enum FolderElement {
    File(Result<LanguageCount, CountFileError>),
    Folder(Result<Vec<DirectoryMember>, FolderError>),
}

fn test() {
    let s = DirectoryMember{
        name: "Test".to_string(),
        ignored: false,
        member: FolderElement::File(Ok(LanguageCount {
            name: "Java".to_string(),
            l_type: LanguageType::General,
            count: Count{
                comment_count: 0,
                code_count: 0,
                empty_count: 0,
            }
        })),
    };
    // let s2 = DirectoryMember {
    //     name: "Test2".to_string(),
    //     ignored: false,
    //     member: FolderElement::Folder(
    //         Ok(vec![DirectoryMember{
    //             name: "Dir2".to_string(),
    //             ignored: false,
    //             member: (),
    //         }])
    //     ),
    // };
}

impl LocTotal for FolderElement {
    fn total(self) -> u32 {
        let t = self.total_types();
        return (t.empty_count + t.comment_count + t.code_count).clone();
    }

    fn total_types(self) -> Count {
        // TODO make it based on total_langs
        let total_langs = self.total_langs();
        let mut total: Count = Default::default();
        for lang in total_langs {
            total += lang.count;
        }
        total
    }

    fn total_langs(self) -> Vec<LanguageCount> {
        let mut language_grouping: HashMap<String, LanguageCount> = Default::default();
        match self {
            FolderElement::File(Err(_)) => {}
            FolderElement::Folder(Err(_)) => {}
            FolderElement::File(Ok(l)) => {
                //Get and Add
                if let Some(old) = language_grouping.get(&l.name) {
                    let new = LanguageCount {
                        name: l.name.clone(),
                        l_type: l.l_type,
                        count: old.count.clone() + l.count,
                    };
                    language_grouping.insert(l.name, new);
                } else {
                    language_grouping.insert(l.name.clone(), l);
                }
            }
            FolderElement::Folder(Ok(languages)) => {
                for dMember in languages {
                    let languages = dMember.member.total_langs();
                    for l in languages {
                        //Get and Add
                        if let Some(old) = language_grouping.get(&l.name) {
                            let new = LanguageCount {
                                name: l.name.clone(),
                                l_type: l.l_type,
                                count: old.count.clone() + l.count,
                            };
                            language_grouping.insert(l.name, new);
                        } else {
                            language_grouping.insert(l.name.clone(), l);
                        }
                    }
                }
            }
        }
        language_grouping.values().cloned().collect::<Vec<LanguageCount>>()
    }
}

pub enum FolderError {
    FailedRead(io::Error)
}

impl Display for FolderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FolderError::FailedRead(e) => write!(f, "Failed to Read Folder: {}", e),
        }
    }
}

pub enum CountFileError {
    LanguageNotFoundError(String),
    NoFileExtension,
    EncodingNotSupported,
    IoError(io::Error),
}

impl Display for CountFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CountFileError::LanguageNotFoundError(ext) => write!(f, "No Lang found for: \".{}\"",ext),
            CountFileError::NoFileExtension => write!(f, "Missing file extension!"),
            CountFileError::EncodingNotSupported => write!(f, "Unsupported Encoding! (Not UTF-8)"),
            IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl From<io::Error> for CountFileError {
    fn from(value: io::Error) -> Self {
        return IoError(value);
    }
}


#[derive(Clone)]
pub struct LanguageCount {
    pub name: String,
    pub l_type: LanguageType,
    pub count: Count,
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

#[derive(Clone)]
pub enum LanguageType {
    General,
    Config,
    Text,
    Template,
    DSL, //Domain Specific Language (SQL, JDSL)
}
