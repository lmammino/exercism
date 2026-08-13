use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Clone, Debug, Default)]
pub struct School {
    rosters: HashMap<u32, BTreeSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // if the students already exists do nothing
        if self.students.contains(student) {
            return;
        }

        self.rosters
            .entry(grade)
            .or_default()
            .insert(student.to_string());

        self.students.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.rosters.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.rosters
            .get(&grade)
            .unwrap_or(&BTreeSet::default())
            .iter()
            .cloned()
            .collect()
    }
}
