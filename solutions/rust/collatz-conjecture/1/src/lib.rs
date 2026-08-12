pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    };

    let mut i = 0;
    let mut n = n;
    loop {
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }

        i += 1;
    }
    Some(i)
}
