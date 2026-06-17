/// Player identity as reported by game engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

impl TryFrom<u8> for Player {
    type Error = String;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Player::P1),
            2 => Ok(Player::P2),
            _ => Err(format!("invalid player number: {val}")),
        }
    }
}

impl Player {
    /// Returns the two characters representing this player on the grid.
    /// P1 → ('@', 'a')   P2 → ('$', 's')
    pub fn chars(self) -> (char, char) {
        match self {
            Player::P1 => ('@', 'a'),
            Player::P2 => ('$', 's'),
        }
    }
}

/// A single cell on the Anfield
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player1Recent, // 'a'
    Player1Old,    // '@'
    Player2Recent, // 's'
    Player2Old,    // '$'
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Player1Old),
            'a' => Ok(Cell::Player1Recent),
            '$' => Ok(Cell::Player2Old),
            's' => Ok(Cell::Player2Recent),
            _ => Err(format!("invalid cell char: '{}'", c)),
        }
    }
}

impl Cell {
    pub fn from_char(c: char) -> Self {
        Self::try_from(c).unwrap_or(Cell::Empty)
    }

    /// Returns true if this cell belongs to the given player
    pub fn belongs_to(self, player: Player) -> bool {
        match (self, player) {
            (Cell::Player1Old | Cell::Player1Recent, Player::P1) => true,
            (Cell::Player2Old | Cell::Player2Recent, Player::P2) => true,
            _ => false,
        }
    }
}

/// 2D grid representing the Anfield
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Cell>>,
}

/// A point in the grid (row, col) — row is Y, col is X in output.
/// Uses signed `i32` to support negative placement offsets (piece padding).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

/// A random game piece
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub rows: usize,
    pub cols: usize,
    /// List of filled cell positions relative to the piece's top-left corner.
    /// Each point is (row_offset, col_offset).
    pub blocks: Vec<(usize, usize)>,
}

/// Full game state for the current turn
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub me: Player,
    pub opponent: Player,
    pub grid: Grid,
    pub piece: Piece,
}
