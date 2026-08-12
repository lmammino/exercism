use std::collections::HashSet;

fn norm_word(word: &str) -> Vec<char> {
    let mut norm = word.to_lowercase().chars().collect::<Vec<char>>();
    norm.sort();
    norm
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let norm = norm_word(word);
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for candidate in possible_anagrams {
        if norm_word(candidate) == norm && word.to_lowercase() != candidate.to_lowercase() {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
