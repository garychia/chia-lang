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

impl ToString for Position {
    fn to_string(&self) -> String {
        format!(
            "(Line: {}, Column: {}, Index: {})",
            self.line, self.column, self.index
        )
    }
}

impl ToString for PositionRange {
    fn to_string(&self) -> String {
        format!(
            "{{Start: {}, End: {}}}",
            self.start.to_string(),
            self.end.to_string()
        )
    }
}
