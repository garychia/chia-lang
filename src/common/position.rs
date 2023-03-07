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

#[cfg(test)]
mod tests {
    use super::{Position, PositionRange};

    #[test]
    fn test_position_new() {
        let pos = Position::new();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
        assert_eq!(pos.index, 0);
    }

    #[test]
    fn test_position_to_string() {
        let pos = Position {
            line: 13,
            column: 23,
            index: 50,
        };
        assert_eq!(
            pos.to_string(),
            format!(
                "(Line: {}, Column: {}, Index: {})",
                pos.line, pos.column, pos.index
            )
        );
        let pos = Position {
            line: 812,
            column: 343,
            index: 903,
        };
        assert_eq!(
            pos.to_string(),
            format!(
                "(Line: {}, Column: {}, Index: {})",
                pos.line, pos.column, pos.index
            )
        );
    }

    #[test]
    fn test_position_range_to_string() {
        let pos1 = Position {
            line: 32,
            column: 43,
            index: 103,
        };
        let pos2 = Position {
            line: 90,
            column: 18,
            index: 190,
        };
        let range = PositionRange {
            start: pos1,
            end: pos2,
        };
        assert_eq!(
            range.to_string(),
            format!(
                "{{Start: {}, End: {}}}",
                range.start.to_string(),
                range.end.to_string()
            )
        );
    }
}
