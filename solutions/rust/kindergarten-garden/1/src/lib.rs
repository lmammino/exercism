use std::{collections::HashMap, sync::LazyLock};

static KIDS_MAP: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ]
    .iter()
    .enumerate()
    .map(|(i, k)| (k.to_owned(), i * 2))
    .collect()
});

static PLANTS_MAP: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ])
});

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let chars: Vec<Vec<char>> = diagram
        .split_ascii_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let kid_idx = KIDS_MAP.get(student).expect("Kid not found");

    [(0_usize, 0_usize), (1, 0), (0, 1), (1, 1)]
        .iter()
        .map(|(dx, dy)| {
            let plant_char = chars[*dy][kid_idx + dx];
            *(PLANTS_MAP.get(&plant_char).expect("Plant not found"))
        })
        .collect()
}
