/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean_isbn = isbn.replace('-', "");
    if clean_isbn.len() != 10 {
        return false;
    }

    let mut check = 0;

    for (i, c) in clean_isbn.chars().rev().enumerate() {
        let d = if i == 0 && c == 'X' {
            10
        } else if c.is_ascii_digit() {
            c.to_digit(10).unwrap()
        } else {
            return false; // invalid char
        };

        check += d * (i as u32 + 1);
    }

    check.rem_euclid(11) == 0
}
