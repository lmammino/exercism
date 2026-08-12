use std::collections::HashSet;


pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();
    
    for n in factors {
        if *n > 0 {
            let mut last = *n;
            while last < limit {
                set.insert(last);
                last = last + n;
            }
        }
    }

    set.iter().sum()
}
