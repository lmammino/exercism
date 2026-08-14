use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::new();

    for c in candidate.chars() {
        if c.is_ascii_alphabetic() {
            let c_lower = c.to_ascii_lowercase();
            if seen.contains(&c_lower) {
                return false;
            }
            seen.insert(c_lower);
        }
    }

    true
}
