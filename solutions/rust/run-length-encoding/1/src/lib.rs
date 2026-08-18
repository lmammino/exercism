use std::fmt::Write;

pub fn encode(source: &str) -> String {
    let mut prev: Option<char> = None;
    let mut count: usize = 0;
    let mut res: String = String::new();

    for c in source.chars() {
        match prev {
            None => {
                prev = Some(c);
                count += 1;
                continue;
            }
            Some(p) if p == c => {
                count += 1;
            }
            Some(p) => {
                if count > 1 {
                    write!(&mut res, "{count}").expect("failed write");
                }
                write!(&mut res, "{p}").expect("failed write");
                prev = Some(c);
                count = 1;
            }
        }
    }

    // deal with any remainder
    if let Some(p) = prev {
        if count > 1 {
            write!(&mut res, "{count}").expect("failed write");
        }
        write!(&mut res, "{p}").expect("failed write");
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut rep = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            write!(rep, "{c}").expect("failed write");
        } else {
            let rep_n: usize = if rep.is_empty() {
                1
            } else {
                rep.parse().unwrap()
            };
            write!(res, "{}", c.to_string().repeat(rep_n)).expect("failed write");
            rep = String::new();
        }
    }

    res
}
