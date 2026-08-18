pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut prev: Option<char> = None;
    let mut count: usize = 0;

    for c in source.chars() {
        if prev == Some(c) {
            count += 1;
        } else {
            flush(&mut result, prev, count);
            prev = Some(c);
            count = 1;
        }
    }
    flush(&mut result, prev, count);
    result
}

fn flush(result: &mut String, ch: Option<char>, count: usize) {
    if let Some(ch) = ch {
        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(ch);
    }
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count: usize = 0;

    for c in source.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            let n = if count == 0 { 1 } else { count };
            for _ in 0..n {
                result.push(c);
            }
            count = 0;
        }
    }

    result
}