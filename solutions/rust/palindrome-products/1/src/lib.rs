use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    // The smallest palindrome product is at least min*min and the largest at
    // most max*max, so the two searches are bracketed by min² and max².
    let smallest = search_palindrome(min, max, min * min, max * max);
    let largest = search_palindrome(min, max, max * max, min * min);

    match (smallest, largest) {
        (Some(s), Some(l)) => Some((s, l)),
        _ => None,
    }
}

// Walks palindromes from `start` toward `end`, returning the first one that has
// a factor pair within [min, max].
fn search_palindrome(min: u64, max: u64, start: u64, end: u64) -> Option<Palindrome> {
    // Direction is fully determined by the start/end comparison:
    // ascending when searching for the smallest, descending for the largest.
    let ascending = start <= end;

    // min*min / max*max themselves need not be palindromes (e.g. min = 10 -> 100), so
    // normalize the start to the nearest palindrome in the search direction:
    //   ascending  -> smallest palindrome >= start
    //   descending -> largest palindrome <= start
    let mut p = if ascending {
        next_palindrome(start.saturating_sub(1))
    } else {
        prev_palindrome(start.saturating_add(1))
    };

    while (ascending && p <= end) || (!ascending && p >= end) {
        let pairs = factor_pairs_in_range(p, min, max);
        if !pairs.is_empty() {
            return Some(Palindrome {
                value: p,
                factors: pairs,
            });
        }
        p = if ascending {
            next_palindrome(p)
        } else {
            prev_palindrome(p)
        };
    }
    None
}

// All factor pairs (f, p/f) with both factors in [min, max], normalized to f <= p/f.
fn factor_pairs_in_range(p: u64, min: u64, max: u64) -> HashSet<(u64, u64)> {
    let mut pairs = HashSet::new();

    // A valid factor f must satisfy: min <= f <= max, min <= p/f <= max, f <= p/f.
    //   f >= ceil(p/max)  -> p/f <= max          (lower bound)
    //   f <= sqrt(p)      -> p/f >= f            (avoids (a,b) and (b,a))
    //   p >= min²         -> sqrt(p) >= min      -> p/f >= min comes for free
    let lo = min.max(p.div_ceil(max));
    let mut f = lo;
    while f <= max && f * f <= p {
        if p.is_multiple_of(f) {
            pairs.insert((f, p / f));
        }
        f += 1;
    }
    pairs
}

// Smallest palindrome strictly greater than n.
fn next_palindrome(n: u64) -> u64 {
    let s = n.to_string();
    let l = s.len();

    // Left half, middle digit included when the length is odd:
    //   n = 1234  -> "12",   n = 12345 -> "123"
    let h1 = &s[0..l.div_ceil(2)];

    // Mirror the left half onto the right, skipping the middle digit for odd
    // lengths so the result always has exactly l digits:
    //   "12" -> "1221",   "123" -> "12321"
    let h2: String = h1[..l / 2].chars().rev().collect();
    let cand: u64 = format!("{h1}{h2}").parse().unwrap();

    // If the mirror already lands strictly above n, it's the answer.
    if cand > n {
        return cand;
    }

    // Otherwise bump the left half up and re-mirror:
    //   n = 1234 -> h1 = "12", cand = 1221 <= n -> h1' = "13" -> "1331"
    let h1_next = h1.parse::<u64>().unwrap() + 1;

    // All-9s half grows a digit ("99" -> "100"): answer is 10^L + 1.
    //   n = 999 -> 1001
    if h1_next.to_string().len() > h1.len() {
        return 10_u64.pow(l as u32) + 1;
    }

    // Re-mirror the bumped half (same skip-the-middle rule):
    //   "13" -> "1331",   "124" -> "12421"
    let h1_next = h1_next.to_string();
    let h2_next: String = h1_next[..l / 2].chars().rev().collect();
    format!("{h1_next}{h2_next}").parse().unwrap()
}

// Largest palindrome strictly less than n.
fn prev_palindrome(n: u64) -> u64 {
    let s = n.to_string();
    let l = s.len();

    // Left half, middle digit included when the length is odd.
    let h1 = &s[0..l.div_ceil(2)];

    // Mirror onto the right side, skipping the middle digit for odd lengths.
    let h2: String = h1[..l / 2].chars().rev().collect();
    let cand: u64 = format!("{h1}{h2}").parse().unwrap();

    // If the mirror lands strictly below n, it's the answer.
    if cand < n {
        return cand;
    }

    // Otherwise bump the left half down and re-mirror.
    let h1_prev = h1.parse::<u64>().unwrap().saturating_sub(1);

    // Half collapsed to zero ("1" -> 0) or lost a digit ("10" -> "9"):
    // the answer is all 9s of length L-1.
    //   prev(10) = 9,   prev(1000) = 999
    if h1_prev == 0 || h1_prev.to_string().len() < h1.len() {
        return 10_u64.pow(l as u32 - 1) - 1;
    }

    // Re-mirror the bumped-down half (same skip-the-middle rule).
    let h1_prev = h1_prev.to_string();
    let h2_prev: String = h1_prev[..l / 2].chars().rev().collect();
    format!("{h1_prev}{h2_prev}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(n: u64) -> bool {
        let s = n.to_string();
        s == s.chars().rev().collect::<String>()
    }

    #[test]
    fn next_palindrome_spot_checks() {
        // (input, expected) — expected is the smallest palindrome strictly > input
        let cases = [
            (0, 1), // degenerate: next(0) = 1
            (1, 2),
            (5, 6),
            (9, 11), // carry: single digit -> two digits
            (10, 11),
            (11, 22), // input is a palindrome -> must bump
            (12, 22),
            (99, 101), // carry: all-9s half grows a digit
            (100, 101),
            (999, 1001),
            (1000, 1001),
            (1234, 1331),   // mirror too low -> bump half
            (1991, 2002),   // input is a palindrome -> bump half
            (12321, 12421), // odd length, middle digit skipped on mirror
            (12921, 13031),
            (9999, 10001),
        ];
        for (n, expected) in cases {
            assert_eq!(next_palindrome(n), expected, "next_palindrome({n})");
        }
    }

    #[test]
    fn prev_palindrome_spot_checks() {
        // (input, expected) — expected is the largest palindrome strictly < input
        let cases = [
            (1, 0), // degenerate: prev(1) = 0
            (2, 1),
            (5, 4),
            (9, 8),
            (10, 9), // zero-half edge: prev(10) = 9
            (11, 9), // input is a palindrome -> must bump down
            (12, 11),
            (100, 99), // borrow: half "10" -> "9"
            (1000, 999),
            (1001, 999), // input is a palindrome -> all-9s of length L-1
            (1234, 1221),
            (2002, 1991),
            (12321, 12221), // odd length, middle digit skipped on mirror
            (10001, 9999),
        ];
        for (n, expected) in cases {
            assert_eq!(prev_palindrome(n), expected, "prev_palindrome({n})");
        }
    }

    #[test]
    fn next_palindrome_is_a_palindrome_strictly_above() {
        for n in 0..=200_000u64 {
            let p = next_palindrome(n);
            assert!(
                is_palindrome(p),
                "next_palindrome({n}) = {p} is not a palindrome"
            );
            assert!(p > n, "next_palindrome({n}) = {p} is not strictly greater");
        }
    }

    #[test]
    fn prev_palindrome_is_a_palindrome_strictly_below() {
        for n in 1..=200_000u64 {
            let p = prev_palindrome(n);
            assert!(
                is_palindrome(p),
                "prev_palindrome({n}) = {p} is not a palindrome"
            );
            assert!(p < n, "prev_palindrome({n}) = {p} is not strictly less");
        }
    }

    #[test]
    fn next_palindrome_enumerates_all_palindromes() {
        // Walking next_palindrome from 0 must yield exactly the full list of
        // palindromes in ascending order — i.e. it never skips or duplicates one.
        let limit = 100_000u64;
        let mut walked = Vec::new();
        let mut p = 0;
        while p <= limit {
            walked.push(p);
            p = next_palindrome(p);
        }
        let direct: Vec<u64> = (0..=limit).filter(|&n| is_palindrome(n)).collect();
        assert_eq!(walked, direct);
    }

    #[test]
    fn prev_palindrome_enumerates_all_palindromes_descending() {
        // Walking prev_palindrome down from the first palindrome below `limit`
        // must yield exactly the full list of palindromes in descending order.
        let limit = 100_000u64;
        let mut walked = Vec::new();
        let mut p = prev_palindrome(limit);
        loop {
            walked.push(p);
            if p == 0 {
                break;
            }
            p = prev_palindrome(p);
        }
        let direct: Vec<u64> = (0..=limit).rev().filter(|&n| is_palindrome(n)).collect();
        assert_eq!(walked, direct);
    }
}
