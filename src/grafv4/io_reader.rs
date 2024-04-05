use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use crate::grafv4::io_reader::ReadFileError::{EncodingNotSupported, IoError};

pub(crate) enum ReadFileError {
    IoError(io::Error),
    EncodingNotSupported,
}

impl Display for ReadFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError(e) => write!(f, "IoError: {e}"),
            EncodingNotSupported => write!(f, "EncodingNotSupported: Non Unicode"),
        }
    }
}

pub(crate) fn read_file(path: &PathBuf) -> Result<Vec<String>, ReadFileError> {
    assert!(path.is_absolute(), "Received Filepath is not absolut! {}", path.display());
    assert!(path.exists(), "No File/Folder exists at this Path: {}", path.display());

    let file = match File::open(&path) {
        Ok(v) => v,
        Err(e) => { return Err(IoError(e)); }
    };

    let lines = BufReader::new(file).lines();
    let mut line_vec: Vec<String> = Vec::with_capacity(lines.size_hint().0);
    for line_result in lines {
        let Ok(line) = line_result else {
            return Err(EncodingNotSupported);
        };
        line_vec.push(line)
    }
    Ok(line_vec)
}

pub(crate) fn read_dir(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let members = path.read_dir()?;
    Ok(members.map(|x| x.unwrap().path()).collect())
}