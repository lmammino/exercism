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

    println!("{phrase} -> {res:?}");

    res.iter().map(|c| c.to_ascii_uppercase()).collect()

    // let mut parts: Vec<String> = Vec::new();
    // let mut last_split_idx = 0;
    // chars.windows(2).enumerate().for_each(|(idx, items)| {
    //     let is_sep = is_separator(items[0]);
    //     if is_sep || (items[0].is_ascii_lowercase() && items[1].is_ascii_uppercase()) {
    //         parts.push(phrase[last_split_idx..(idx + 1)].to_owned());
    //         last_split_idx = idx + 1;
    //     }
    // });

    // if last_split_idx < chars.len() {
    //     parts.push(phrase[last_split_idx..(chars.len())].to_owned());
    // }

    // println!("{phrase} -> {parts:?}");

    // parts
    //     .iter()
    //     .map(|w| w.chars().next().unwrap().to_ascii_uppercase())
    //     .collect()
}
