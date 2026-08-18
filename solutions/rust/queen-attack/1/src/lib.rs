#[derive(Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl TryFrom<Pos> for ChessPosition {
    type Error = ();

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        if value.x >= 0 && value.y >= 0 && value.x <= 7 && value.y <= 7 {
            return Ok(ChessPosition {
                x: value.x as u8,
                y: value.y as u8,
            });
        }

        Err(())
    }
}

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
        Pos::new(rank, file).try_into().ok()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // same rank or file
        if self.pos.x == other.pos.x || self.pos.y == other.pos.y {
            return true;
        }

        // can attack on a diagonal?
        let x1 = self.pos.x;
        let x2 = other.pos.x;
        let y1 = self.pos.y;
        let y2 = other.pos.y;

        let delta = match (x1, x2, y1, y2) {
            // case 1. going up-left
            (x1, x2, y1, y2) if x1 > x2 && y1 > y2 => Some((-1, -1)),
            // case 2. going down-left
            (x1, x2, y1, y2) if x1 > x2 && y1 < y2 => Some((-1, 1)),
            // case 3. going up-right
            (x1, x2, y1, y2) if x1 < x2 && y1 > y2 => Some((1, -1)),
            // case 4. going down-right
            (x1, x2, y1, y2) if x1 < x2 && y1 < y2 => Some((1, 1)),
            _ => None,
        };

        if let Some((dx, dy)) = delta {
            // walk until reaching the other queen (can attack) or the edge of the board (can't attack)
            let mut last_pos = self.pos.clone();
            loop {
                let candidate_next: Result<ChessPosition, _> =
                    Pos::new(last_pos.x as i32 + dx, last_pos.y as i32 + dy).try_into();
                match candidate_next {
                    Err(_) => return false,
                    Ok(pos) => {
                        if pos == other.pos {
                            return true;
                        }

                        last_pos = pos;
                    }
                }
            }
        }

        // any other position (can't attack)
        false
    }
}
