pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = (num as f64).log10().floor() as u32 + 1;

    let mut acc = 0;
    let mut dec = num;
    for i in (0..num_digits).rev() {
        let current_factor = 10u32.pow(i);
        let current_digit = dec / current_factor;
        dec -= current_digit * current_factor;
        acc += current_digit.pow(num_digits);
    }

    acc == num
}
