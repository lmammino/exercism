#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    };
    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }

    // if empty or all zeros, return 0
    if number.is_empty() || number.iter().all(|d| *d == 0) {
        return Ok(vec![0]);
    }

    // validate input digits
    for d in number {
        if *d >= from_base {
            return Err(Error::InvalidDigit(*d));
        }
    }

    let base10_n = digits_to_base10_number(number, from_base);
    Ok(base10_to_base_n(base10_n, to_base))
}

fn digits_to_base10_number(number: &[u32], from_base: u32) -> u32 {
    number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, curr)| acc + curr * from_base.pow(i as u32))
}

fn base10_to_base_n(base10_n: u32, to_base: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut q = base10_n;
    while q > 0 {
        let r = q.rem_euclid(to_base);
        q /= to_base;
        res.push(r);
    }

    res.iter().rev().cloned().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits_to_base10_number() {
        // b2
        assert_eq!(digits_to_base10_number(&[0], 2), 0);
        assert_eq!(digits_to_base10_number(&[1], 2), 1);
        assert_eq!(digits_to_base10_number(&[1, 0], 2), 2);
        assert_eq!(digits_to_base10_number(&[1, 1], 2), 3);
        assert_eq!(digits_to_base10_number(&[1, 0, 0], 2), 4);
        assert_eq!(digits_to_base10_number(&[1, 0, 1], 2), 5);
        assert_eq!(digits_to_base10_number(&[1, 1, 0], 2), 6);
        assert_eq!(digits_to_base10_number(&[1, 1, 1], 2), 7);
        assert_eq!(digits_to_base10_number(&[1, 0, 0, 0], 2), 8);
        // b10
        assert_eq!(digits_to_base10_number(&[0], 10), 0);
        assert_eq!(digits_to_base10_number(&[1], 10), 1);
        assert_eq!(digits_to_base10_number(&[1, 0], 10), 10);
        assert_eq!(digits_to_base10_number(&[2, 3], 10), 23);
        assert_eq!(digits_to_base10_number(&[1, 9, 8, 7], 10), 1987);
    }
}
