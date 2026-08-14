use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();
    sentence.chars().for_each(|c| {
        if c.is_ascii_alphabetic() {
            letters.insert(c.to_ascii_lowercase());
        }
    });
    letters.len() == 26
}
