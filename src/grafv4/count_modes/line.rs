use crate::grafv4::countable::Countable;

pub(crate) type LineCount = u32;

impl Countable for LineCount {
    fn count(content: Vec<String>, _: &str) -> Box<Self> {
        return Box::from(content.len() as LineCount);
    }
}