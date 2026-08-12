pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    while n > 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            // Note: In a real scenario, one might worry about overflow here.
            n = n * 3 + 1;
        }
        steps += 1;
    }
    Some(steps)
}
