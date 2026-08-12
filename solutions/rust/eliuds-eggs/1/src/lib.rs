pub fn egg_count(display_value: u32) -> usize {
    // display_value.count_ones() // OBVIOUS SOLUTION
    (0..32)
        .filter(|pos| {
            let n = 1 << pos;
            let m = display_value;
            m & n == n
        })
        .count()
}
