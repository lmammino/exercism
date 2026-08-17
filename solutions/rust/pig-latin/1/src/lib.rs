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
    // NOTE, applies the rules in a different order than they are expressed in the problem
    // Rule 1 → vowel / "xr" / "yt" (special exceptions)
    // Rule 3 → consonants + "qu" (more specific than Rule 2)
    // Rule 4 → consonants + "y" (more specific than Rule 2)
    // Rule 2 → general consonant case (the fallback)

    // rule 1: If a word begins with a vowel, or starts with "xr" or "yt", add an "ay" sound to the end of the word.
    if word.starts_with(|c: char| is_vowel(c)) || word.starts_with("xr") || word.starts_with("yt") {
        return format!("{word}ay");
    }

    // rule 3: If a word starts with zero or more consonants followed by "qu", first move those consonants (if any) and
    // the "qu" part to the end of the word, and then add an "ay" sound to the end of the word.
    let cluster = word
        .chars()
        .take_while(|c| is_consonant(*c) && *c != 'q')
        .collect::<String>();
    let without_cluster = &word[(cluster.len())..];

    if without_cluster.starts_with("qu") {
        let remaining = &word[(cluster.len() + 2)..];
        return format!("{remaining}{cluster}quay");
    }

    // Rule 4: If a word starts with one or more consonants followed by "y", first move the consonants preceding the "y"to
    // the end of the word, and then add an "ay" sound to the end of the word.
    let cluster = word
        .chars()
        .take_while(|c| is_consonant(*c) && *c != 'y')
        .collect::<String>();

    let without_cluster = &word[(cluster.len())..];
    if !cluster.is_empty() && without_cluster.starts_with('y') {
        return format!("{without_cluster}{cluster}ay");
    }

    // rule 2: If a word begins with one or more consonants, first move those consonants to the end of the word and then add an
    // "ay" sound to the end of the word.
    let cluster = word
        .chars()
        .take_while(|c| is_consonant(*c))
        .collect::<String>();
    let without_cluster = &word[(cluster.len())..];
    if !cluster.is_empty() {
        return format!("{without_cluster}{cluster}ay");
    }

    word.to_string()
}
