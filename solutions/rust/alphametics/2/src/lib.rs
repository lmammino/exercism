use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Digits(Vec<char>);

impl Deref for Digits {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Digits {
    fn from(value: &str) -> Self {
        Digits(value.chars().collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Expr {
    left: Vec<Digits>,
    right: Digits,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Column {
    // A + A + B becomes [('A', 2), ('B', 1)]
    addends: Vec<(char, u64)>,
    result: Option<char>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NormalizedExpr {
    // Columns are stored right-to-left.
    columns: Vec<Column>,
    leading_vars: HashSet<char>,
    num_vars: usize,
}

impl Expr {
    fn normalize(&self) -> NormalizedExpr {
        let num_columns = self
            .left
            .iter()
            .map(|d| d.len())
            .chain(std::iter::once(self.right.len()))
            .max()
            .unwrap_or(0);

        let columns = (0..num_columns)
            .map(|column| {
                let mut addends = HashMap::new();

                for digits in &self.left {
                    if let Some(var) = column_var(digits, column) {
                        *addends.entry(var).or_insert(0) += 1;
                    }
                }

                let mut addends: Vec<_> = addends.into_iter().collect();
                addends.sort();

                Column {
                    addends,
                    result: column_var(&self.right, column),
                }
            })
            .collect();

        let vars: HashSet<_> = self
            .left
            .iter()
            .flat_map(|d| d.iter())
            .chain(self.right.iter())
            .copied()
            .collect();

        let leading_vars = self
            .left
            .iter()
            .chain(std::iter::once(&self.right))
            .filter(|d| d.len() > 1)
            .map(|d| d[0])
            .collect();

        NormalizedExpr {
            columns,
            leading_vars,
            num_vars: vars.len(),
        }
    }
}

struct Solver<'a> {
    expr: &'a NormalizedExpr,
    candidate_solution: HashMap<char, u8>,
    used_digits: HashSet<u8>,
}

impl Solver<'_> {
    // Solve one decimal column at a time, propagating the carry to the left.
    fn solve_column(&mut self, column: usize, carry: u64) -> bool {
        if column == self.expr.columns.len() {
            return carry == 0;
        }

        self.solve_addends(column, 0, carry)
    }

    fn solve_addends(&mut self, column: usize, addend: usize, sum: u64) -> bool {
        let current_column = &self.expr.columns[column];

        // Once all addends are known, the result digit is forced by the sum.
        if addend == current_column.addends.len() {
            let expected_digit = (sum % 10) as u8;
            let next_carry = sum / 10;

            let Some(result_var) = current_column.result else {
                return expected_digit == 0
                    && self.solve_column(column + 1, next_carry);
            };

            if let Some(&digit) = self.candidate_solution.get(&result_var) {
                return digit == expected_digit
                    && self.solve_column(column + 1, next_carry);
            }

            if self.used_digits.contains(&expected_digit) {
                return false;
            }

            if expected_digit == 0 && self.expr.leading_vars.contains(&result_var) {
                return false;
            }

            self.candidate_solution.insert(result_var, expected_digit);
            self.used_digits.insert(expected_digit);

            if self.solve_column(column + 1, next_carry) {
                return true;
            }

            self.candidate_solution.remove(&result_var);
            self.used_digits.remove(&expected_digit);

            return false;
        }

        let (var, occurrences) = current_column.addends[addend];

        if let Some(&digit) = self.candidate_solution.get(&var) {
            return self.solve_addends(
                column,
                addend + 1,
                sum + digit as u64 * occurrences,
            );
        }

        for digit in 0..=9 {
            if self.used_digits.contains(&digit) {
                continue;
            }

            if digit == 0 && self.expr.leading_vars.contains(&var) {
                continue;
            }

            self.candidate_solution.insert(var, digit);
            self.used_digits.insert(digit);

            if self.solve_addends(
                column,
                addend + 1,
                sum + digit as u64 * occurrences,
            ) {
                return true;
            }

            self.candidate_solution.remove(&var);
            self.used_digits.remove(&digit);
        }

        false
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solve_expr(parse(input).normalize())
}

fn parse(input: &str) -> Expr {
    let (raw_left, raw_right) = input
        .split_once(" == ")
        .expect("' == ' not found in expr");

    let left = raw_left
        .trim()
        .split(" + ")
        .map(Into::into)
        .collect();

    let right = raw_right.trim().into();

    Expr { left, right }
}

fn solve_expr(expr: NormalizedExpr) -> Option<HashMap<char, u8>> {
    if expr.num_vars > 10 {
        return None;
    }

    let mut solver = Solver {
        expr: &expr,
        candidate_solution: HashMap::new(),
        used_digits: HashSet::new(),
    };

    if solver.solve_column(0, 0) {
        Some(solver.candidate_solution)
    } else {
        None
    }
}

// Get the variable in a given decimal column, counting from the right.
fn column_var(digits: &Digits, column: usize) -> Option<char> {
    digits
        .len()
        .checked_sub(column + 1)
        .map(|i| digits[i])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("I + BB == ILL"),
            Expr {
                left: vec!["I".into(), "BB".into()],
                right: "ILL".into(),
            }
        );

        assert_eq!(
            parse("A == B"),
            Expr {
                left: vec!["A".into()],
                right: "B".into(),
            }
        );

        assert_eq!(
            parse("ACA + DD == BD"),
            Expr {
                left: vec!["ACA".into(), "DD".into()],
                right: "BD".into(),
            }
        );
    }

    #[test]
    fn test_column_var() {
        let digits: Digits = "ABC".into();

        assert_eq!(column_var(&digits, 0), Some('C'));
        assert_eq!(column_var(&digits, 1), Some('B'));
        assert_eq!(column_var(&digits, 2), Some('A'));
        assert_eq!(column_var(&digits, 3), None);
    }

    #[test]
    fn test_normalize() {
        let expr = parse("AB + AC + A == DE");
        let normalized = expr.normalize();

        assert_eq!(
            normalized.columns,
            vec![
                Column {
                    addends: vec![('A', 1), ('B', 1), ('C', 1)],
                    result: Some('E'),
                },
                Column {
                    addends: vec![('A', 2)],
                    result: Some('D'),
                },
            ]
        );

        assert_eq!(
            normalized.leading_vars,
            HashSet::from(['A', 'D'])
        );

        assert_eq!(normalized.num_vars, 5);
    }

    #[test]
    fn puzzle_with_three_letters() {
        let expected = [('I', 1), ('B', 9), ('L', 0)]
            .into_iter()
            .collect();

        assert_eq!(solve("I + BB == ILL"), Some(expected));
    }

    #[test]
    fn solution_must_have_unique_value_for_each_letter() {
        assert_eq!(solve("A == B"), None);
    }

    #[test]
    fn leading_zero_solution_is_invalid() {
        assert_eq!(solve("ACA + DD == BD"), None);
    }

    #[test]
    fn puzzle_with_two_digits_final_carry() {
        let expected = [('A', 9), ('B', 1), ('C', 0)]
            .into_iter()
            .collect();

        assert_eq!(
            solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC"),
            Some(expected)
        );
    }

    #[test]
    fn puzzle_with_four_letters() {
        let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
            .into_iter()
            .collect();

        assert_eq!(solve("AS + A == MOM"), Some(expected));
    }

    #[test]
    fn puzzle_with_six_letters() {
        let expected = [
            ('N', 7),
            ('O', 4),
            ('T', 9),
            ('L', 1),
            ('A', 0),
            ('E', 2),
        ]
        .into_iter()
        .collect();

        assert_eq!(solve("NO + NO + TOO == LATE"), Some(expected));
    }

    #[test]
    fn puzzle_with_seven_letters() {
        let expected = [
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            solve("HE + SEES + THE == LIGHT"),
            Some(expected)
        );
    }

    #[test]
    fn puzzle_with_eight_letters() {
        let expected = [
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            solve("SEND + MORE == MONEY"),
            Some(expected)
        );
    }

    #[test]
    fn puzzle_with_ten_letters() {
        let expected = [
            ('A', 5),
            ('D', 3),
            ('E', 4),
            ('F', 7),
            ('G', 8),
            ('N', 0),
            ('O', 2),
            ('R', 1),
            ('S', 6),
            ('T', 9),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE"),
            Some(expected)
        );
    }

    #[test]
    fn more_than_ten_variables_has_no_solution() {
        assert_eq!(
            solve("ABCDEF + GHIJK == LMNOP"),
            None
        );
    }
}
