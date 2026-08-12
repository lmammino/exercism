pub fn abbreviate(phrase: &str) -> String {
    let chars: Vec<char> = phrase.chars().collect();
    let mut prev = chars[0];
    let mut res: Vec<char> = Vec::from([prev]);
    for c in &chars[1..] {
        if ((prev == ' ' || prev == '-' || (prev.is_ascii_lowercase() && c.is_ascii_uppercase()))
            && *c != ' '
            && *c != '-'
            && *c != '_')
            || (prev == '_' && c.is_ascii_uppercase())
        {
            res.push(*c);
        }
        prev = *c;
    }

    res.iter().map(|c| c.to_ascii_uppercase()).collect()
}
