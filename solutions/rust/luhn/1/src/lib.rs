/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits = Vec::with_capacity(16);
    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        }

        if let Some(d) = c.to_digit(10) {
            digits.push(d);
        } else if !c.is_whitespace() {
            return false;
        }
    }

    if digits.len() <= 1 {
        return false;
    }

    let digits_doubled = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if (i + 1) % 2 == 0 {
                let mut new_d = *d * 2;
                if new_d > 9 {
                    new_d -= 9;
                }
                return new_d;
            }
            *d
        })
        .rev()
        .collect::<Vec<u32>>();

    let sum = digits_doubled.iter().sum::<u32>();
    let check = (10 - (sum % 10)) % 10;

    sum % 10 == 0 || check == *digits.last().unwrap()
}
