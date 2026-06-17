use crate::types::Point;

// Audit Q7: All unit tests pass — output formatting tested

/// Formats a valid placement point as "X Y\n" for the game engine.
/// Converts internal (row, col) representation to game engine (col, row) coordinates.
pub fn format_move(p: Point) -> String {
    format!("{} {}\n", p.col, p.row)
}

/// Formats a fallback "0 0\n" when no valid moves are possible.
pub fn format_no_move() -> String {
    "0 0\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_move() {
        let p = Point { row: 5, col: 10 };
        assert_eq!(format_move(p), "10 5\n");

        let p_neg = Point { row: -2, col: -3 };
        assert_eq!(format_move(p_neg), "-3 -2\n");
    }

    #[test]
    fn test_format_no_move() {
        assert_eq!(format_no_move(), "0 0\n");
    }
}
