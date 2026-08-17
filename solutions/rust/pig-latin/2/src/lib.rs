pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_consonant(c: char) -> bool {
    !is_vowel(c) && c.is_ascii_alphabetic()
}

fn translate_word(word: &str) -> String {
    // Rule 1: vowel / "xr" / "yt" → no prefix moved
    if word.starts_with(|c: char| is_vowel(c)) || word.starts_with("xr") || word.starts_with("yt") {
        return format!("{word}ay");
    }

    // Find the consonant cluster to move.
    // 'y' is a consonant only at position 0; after that it acts as a vowel.
    let cluster: String = word
        .chars()
        .enumerate()
        .take_while(|(i, c)| is_consonant(*c) && (*c != 'y' || *i == 0))
        .map(|(_, c)| c)
        .collect();

    let mut split = cluster.len();

    // Rule 3: if the cluster ends with 'q' and is followed by 'u',
    // include the 'u' in the moved prefix ("qu" moves together).
    if cluster.ends_with('q') && word[split..].starts_with('u') {
        split += 1;
    }

    // Rules 2 & 4: move the prefix to the end and add "ay".
    format!("{}{}ay", &word[split..], &word[..split])
}