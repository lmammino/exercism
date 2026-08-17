use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let f = proper_divisors(num);
    let aliquot_sum: u64 = f.iter().sum();
    dbg!(num, f, aliquot_sum);
    Some(match num.cmp(&aliquot_sum) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Deficient,
        Ordering::Less => Classification::Abundant,
    })
}

/// Returns all divisors of `n` excluding `n` itself
fn proper_divisors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n.is_multiple_of(i) {
            if i != n {
                result.push(i);
            }
            let other = n / i;
            if other != i && other != n {
                result.push(other);
            }
        }
        i += 1;
    }
    result.sort();
    result
}
