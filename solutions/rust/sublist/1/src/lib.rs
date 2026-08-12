#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_included_at(shorter: &[i32], longer: &[i32], start_idx: usize) -> bool {
    for (i, v) in shorter.iter().enumerate() {
        if longer[i + start_idx] != *v {
            return false;
        }
    }
    true
}

fn is_included(shorter: &[i32], longer: &[i32]) -> bool {
    let r = 0..=(longer.len() - shorter.len());
    r.into_iter()
        .any(|start_idx| is_included_at(shorter, longer, start_idx))
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,     // both empty: equal
        (0, _) => Comparison::Sublist,   // first empty: sublist
        (_, 0) => Comparison::Superlist, // second empty: superlist
        (l1, l2) => {
            let shorter = if l1 < l2 { &first_list } else { &second_list };
            let longer = if l1 < l2 { &second_list } else { &first_list };

            if !is_included(shorter, longer) {
                return Comparison::Unequal;
            }

            if l1 == l2 {
                Comparison::Equal
            } else if l1 > l2 {
                Comparison::Superlist
            } else {
                Comparison::Sublist
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_included_at() {
        assert!(is_included_at(&[1, 2, 3, 4], &[1, 2, 3, 4], 0));
        assert!(is_included_at(&[1, 2, 3], &[1, 2, 3, 4], 0));
        assert!(is_included_at(&[2, 3], &[1, 2, 3, 4], 1));
        assert!(is_included_at(&[3, 4], &[1, 2, 3, 4], 2));
    }

    #[test]
    fn test_is_included() {
        assert!(is_included(&[1, 2, 3, 4], &[1, 2, 3, 4]));
        assert!(is_included(&[1, 2, 3], &[1, 2, 3, 4]));
        assert!(is_included(&[2, 3], &[1, 2, 3, 4]));
        assert!(is_included(&[3, 4], &[1, 2, 3, 4]));
    }
}
