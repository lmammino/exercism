#[derive(Debug, Clone, PartialEq)]
pub struct ChessPosition {
    x: u8, // x pos or "rank"
    y: u8, // y pos or "file"
}

#[derive(Debug, Clone)]
pub struct Queen {
    pub pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            Some(ChessPosition {
                x: rank as u8,
                y: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // Math shortcut:
        //   Two queens are on the same diagonal iff the absolute
        //   differences of their coordinates are equal!
        let dx = self.pos.x as i8 - other.pos.x as i8;
        let dy = self.pos.y as i8 - other.pos.y as i8;
        // same row, same column, or same diagonal
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}
