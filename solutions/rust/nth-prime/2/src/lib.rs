use std::convert::TryFrom;

// https://www.geeksforgeeks.org/sieve-of-eratosthenes/
fn build_sieve_of_eratosthenes (max: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for _ in 0..=max {
        primes.push(true);
    }

    let mut p = 2;
    while p*p < max {
        let p_us = usize::try_from(p).unwrap();
        if primes[p_us] == true {
            for i in (p*p..max+1).step_by(p_us) {
                let i_us = usize::try_from(i).unwrap();
                primes[i_us] = false
            }
        }

        p += 1
    }

    primes
      .iter()
      .enumerate() // add iteration indices
      .skip(2)  // skip the first 2 numbers (0, 1)
      .filter(|&(_, val)| *val) // keeps only the values that are marked as true in the vector of primes
      .map(|(i, _)| i as u32) // converts every remaining number to it's position (e.g. 3rd True will become `3`)
      .collect()
}


pub fn nth(n: u32) -> u32 {
    let sieve = build_sieve_of_eratosthenes(300_000);
    let n_us = usize::try_from(n).unwrap();
    sieve[n_us]
}
