use std::ops::Add;

const COUNT_LITERAL: &[&str] = &[
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn b(idx: usize) -> String {
    if idx == 1 {
        "bottle".to_string()
    } else {
        "bottles".to_string()
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    let first = (chars.next().map(|c| c.to_ascii_uppercase())).unwrap_or_default();
    let next: String = chars.collect();
    format!("{}{}", first, next)
}

fn verse(idx: usize) -> String {
    let n = capitalize(COUNT_LITERAL[idx]);
    let n_bottle = b(idx);
    let n_minus_1 = COUNT_LITERAL[idx - 1];
    let n_bottle_minus_1 = b(idx - 1);
    format!("{n} green {n_bottle} hanging on the wall,\n")
        .repeat(2)
        .add("And if one green bottle should accidentally fall,\n")
        .add(
            format!("There'll be {n_minus_1} green {n_bottle_minus_1} hanging on the wall.")
                .as_str(),
        )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..(take_down as usize))
        .map(|n| verse(start_bottles as usize - n))
        .collect::<Vec<String>>()
        .join("\n\n")
}
