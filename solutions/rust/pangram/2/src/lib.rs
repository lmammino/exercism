use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let seen_chars: HashSet<char> = HashSet::from_iter(
        sentence
            .to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic()),
    );
    seen_chars.len() == 26
}
