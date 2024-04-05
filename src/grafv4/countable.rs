use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

pub(crate) trait Countable: Add + Sum + Display + Default + Clone {
    fn count(content: Vec<String>, extension: &str) -> Box<Self>;
}