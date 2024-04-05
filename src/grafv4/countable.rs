use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

pub(crate) trait Countable: Add + Sum + Display + Default + Clone {
    fn count(_: Vec<String>) -> Box<Self>;
}