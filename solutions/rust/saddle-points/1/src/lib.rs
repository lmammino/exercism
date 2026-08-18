pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = vec![];

    if input.is_empty() {
        return res;
    }

    for (row_idx, row) in input.iter().enumerate() {
        if row.is_empty() {
            continue;
        }

        let mut iter = row.iter().enumerate();

        let mut current_max = *(iter.next().unwrap().1);
        let mut current_max_idxs = vec![0];

        for (idx, val) in iter {
            if current_max == *val {
                // found another max
                current_max_idxs.push(idx);
            } else if current_max <= *val {
                // found a bigger value
                current_max_idxs = vec![idx];
                current_max = *val;
            }
        }

        // for each candidate idx check if it's the min value in its column
        for col_idx in current_max_idxs {
            let is_min_val_in_col = input.iter().all(|row| row[col_idx] >= current_max);
            if is_min_val_in_col {
                res.push((row_idx, col_idx));
            }
        }
    }

    res
}
