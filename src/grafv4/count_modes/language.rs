use std::collections::{HashMap};
use std::iter::Sum;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::AppState;
use crate::grafv4::countable::Countable;

#[derive(Default, Clone)]
pub(crate) struct LanguageCount(HashMap<String, u32>);

impl Add for LanguageCount {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for val in rhs.0 {
            let start: &u32 = self.0.get(&val.0).unwrap_or(&0);
            self.0.insert(val.0, start + val.1);
        }
        self
    }
}

impl Sum for LanguageCount {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(LanguageCount::default(), |acc, num| acc + num)
    }
}

impl Display for LanguageCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.len() > 1 { return write!(f, ""); }
        let lang_name = self.0.keys().next().unwrap();
        let amount = self.0.values().next().unwrap();
        write!(f, "{}: {}", amount, lang_name)
    }
}

impl Countable for LanguageCount {
    fn count(content: Vec<String>, extension: &str, state: &AppState) -> Result<Self, String> {
        let lang = state.languages.iter()
            .find(|x| x.file_extension == extension)
            .ok_or(format!("LanguageNotFound: .{}", extension))?;
        let mut map: HashMap<String, u32> = HashMap::new();
        map.insert(
            lang.name.clone(),
            content.len() as u32,
        );
        Ok(LanguageCount { 0: map })
    }

    fn display_summary(self, project_name: String) {
        println!("Project: {}:", project_name);
        for d in self.0 {
            println!("  {} -> {} Lines", d.0, d.1);
        }
    }

    fn display_legend() {
        println!("Legend: => Lines: Language")
    }
}

