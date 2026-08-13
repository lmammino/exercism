use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    BTreeMap::from_iter(
        h.iter()
            .flat_map(|(value, chars)| chars.iter().map(|c| (c.to_ascii_lowercase(), *value))),
    )
}
