const D: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbours(pos: (usize, usize), garden: &[&str]) -> u32 {
    let (x, y) = pos;
    let w = garden.get(0).map(|item| item.len()).unwrap_or(0) as isize;
    let h = garden.len() as isize;

    D.iter()
        .filter(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            nx >= 0
                && nx < w
                && ny >= 0
                && ny < h
                && garden[ny as usize].chars().nth(nx as usize).unwrap() == '*'
        })
        .count() as u32
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, cell)| match cell {
                    ' ' => {
                        let neighbours = count_neighbours((x, y), &garden);
                        if neighbours == 0 {
                            return ' ';
                        };
                        char::from_digit(neighbours, 10).unwrap()
                    }
                    '*' => '*',
                    _ => unreachable!("found a char that was not '*' or ' ' in the garden"),
                })
                .collect()
        })
        .collect()
}
