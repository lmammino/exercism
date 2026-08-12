pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    let mut curr = n;
    while divisor <= curr {
        if curr % divisor == 0 {
            curr = curr / divisor;
            factors.push(divisor);
        } else {
            divisor += 1
        }
    }
    factors
}
