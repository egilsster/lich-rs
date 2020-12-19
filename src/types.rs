use ansi_term::Colour::{Green, Red};
use std::{collections::BTreeMap, fmt};

pub type DependencyMap = BTreeMap<String, bool>;
pub type DependencySet = BTreeMap<String, String>;
pub type DependencyList = Vec<String>;

pub struct ComparisonResult {
    pub dependencies: DependencyMap,
    pub is_ok: bool,
}

impl ComparisonResult {
    pub fn new() -> Self {
        ComparisonResult {
            dependencies: DependencyMap::new(),
            is_ok: true,
        }
    }
}

impl fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut report = String::new();
        for (name, &allowed) in self.dependencies.iter() {
            if allowed {
                let entry = format!("{}", Green.paint(format!("\u{2713} {}\n", name)));
                report.push_str(&entry);
            } else {
                let entry = format!("{}", Red.paint(format!("\u{2717} {}\n", name)));
                report.push_str(&entry);
            }
        }

        write!(f, "{}", report)
    }
}
