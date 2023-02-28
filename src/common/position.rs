#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Clone)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

impl Position {
    pub fn new() -> Position {
        Position {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}
