pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.len() > 0 {
        for i in 0..list.len() - 1 {
            let line = format!("For want of a {} the {} was lost.\n", list[i], list[i+1]);
            proverb.push_str(&line.to_string());
        }
        proverb.push_str(&format!("And all for the want of a {}.", list[0]).to_string());
    }

    proverb.to_string()
}
