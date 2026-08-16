#[derive(Debug, Clone)]
pub struct PascalsTriangle(Vec<Vec<u32>>);

// 0 -> [ ]
// 1 -> [  1 (1,0) ]
// 2 -> [  1 (2,0)  1 (2,1) ]
// 3 -> [  1 (3,0)  2 (3,1)  1 (3,2) ]
// 4 -> [  1 (4,0)  3 (4,1)  3 (4,2)  1 (4,3) ]
// 5 -> [  1 (5,0)  4 (5,1)  6 (5,2)  4 (5,3)  1 (5,4) ]
// 6 -> [  1 (6,0)  5 (6,1) 10 (6,2) 10 (6,3)  5 (6,4)  1 (6,5) ]

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self(vec![]);
        }

        let mut rows: Vec<Vec<u32>> = vec![];
        for row_id in 1..=row_count {
            let mut row: Vec<u32> = vec![];
            for cell_id in 0..row_id {
                if cell_id == 0 || cell_id == (row_id - 1) {
                    row.push(1);
                } else {
                    let prev_row = (row_id - 2) as usize;
                    row.push(
                        rows[prev_row][(cell_id - 1) as usize] + rows[prev_row][cell_id as usize],
                    );
                }
            }
            rows.push(row);
        }

        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
