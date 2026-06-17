use std::io::BufRead;
use crate::types::{Player, Cell, Grid, Piece, GameState};

/// Helper — Read a line. Returns error containing "EOF" on end of file.
fn read_line<R: BufRead>(reader: &mut R) -> Result<String, String> {
    let mut buf = String::new();
    let bytes_read = reader.read_line(&mut buf).map_err(|e| e.to_string())?;
    if bytes_read == 0 {
        return Err("EOF".to_string());
    }
    // Trim only trailing newlines to preserve leading/trailing spaces in grid
    Ok(buf.trim_end_matches('\n').trim_end_matches('\r').to_string())
}

/// Parse player ID line (TDD)
pub fn parse_player_line(line: &str) -> Result<Player, String> {
    if !line.starts_with("$$$ exec p") || line.len() < 11 {
        return Err(format!("invalid player line: {line}"));
    }
    match line.as_bytes()[10] {
        b'1' => Ok(Player::P1),
        b'2' => Ok(Player::P2),
        _    => Err(format!("unknown player in line: {line}")),
    }
}

fn parse_anfield_header(line: &str) -> Result<(usize, usize), String> {
    // "Anfield 20 15:"
    let line = line.strip_prefix("Anfield ").ok_or("missing Anfield prefix")?;
    let line = line.strip_suffix(':').ok_or("missing colon")?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("expected 'cols rows' in header".into());
    }
    let cols: usize = parts[0].parse().map_err(|_| "invalid cols")?;
    let rows: usize = parts[1].parse().map_err(|_| "invalid rows")?;
    Ok((cols, rows))
}

/// Parse Anfield grid header & rows
pub fn parse_anfield<R: BufRead>(reader: &mut R) -> Result<Grid, String> {
    let header = read_line(reader)?;
    let (cols, rows) = parse_anfield_header(&header)?;

    // Skip column header line (e.g. "    012345...")
    read_line(reader)?;

    let mut data = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = read_line(reader)?;
        // Strip first 4 chars: "000 " for row number prefix
        if line.len() < 4 + cols {
            return Err(format!("row line too short: expected {}, got {}", 4 + cols, line.len()));
        }
        let row_chars = &line[4..4 + cols];
        let row: Vec<Cell> = row_chars.chars().map(Cell::from_char).collect();
        data.push(row);
    }

    Ok(Grid { rows, cols, data })
}

fn parse_piece_header(line: &str) -> Result<(usize, usize), String> {
    let line = line.strip_prefix("Piece ").ok_or("missing Piece prefix")?;
    let line = line.strip_suffix(':').ok_or("missing colon")?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("expected 'cols rows' in piece header".into());
    }
    let cols: usize = parts[0].parse().map_err(|_| "invalid cols")?;
    let rows: usize = parts[1].parse().map_err(|_| "invalid rows")?;
    Ok((cols, rows))
}

/// Parse random piece header & blocks
pub fn parse_piece<R: BufRead>(reader: &mut R) -> Result<Piece, String> {
    let header = read_line(reader)?;
    let (cols, rows) = parse_piece_header(&header)?;

    let mut blocks = Vec::new();
    for r in 0..rows {
        let line = read_line(reader)?;
        for (c, ch) in line.chars().enumerate() {
            if c >= cols { break; }
            if ch != '.' {
                blocks.push((r, c));
            }
        }
    }

    Ok(Piece { rows, cols, blocks })
}

/// Parses a complete turn from stdin.
pub fn parse_turn<R: BufRead>(
    reader: &mut R,
    state: Option<GameState>,
) -> Result<GameState, String> {
    let (me, opponent) = if let Some(ref s) = state {
        (s.me, s.opponent)
    } else {
        let player_line = read_line(reader)?;
        let me = parse_player_line(&player_line)?;
        let opponent = match me {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
        (me, opponent)
    };

    let grid = parse_anfield(reader)?;
    let piece = parse_piece(reader)?;

    Ok(GameState { me, opponent, grid, piece })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_player_line() {
        assert_eq!(parse_player_line("$$$ exec p1 : [robots/bender]").unwrap(), Player::P1);
        assert_eq!(parse_player_line("$$$ exec p2 : [robots/bender]").unwrap(), Player::P2);
        assert!(parse_player_line("$$$ exec pX : [whatever]").is_err());
        assert!(parse_player_line("").is_err());
    }

    #[test]
    fn test_parse_anfield() {
        let input = "Anfield 3 2:\n    012\n000 .@.\n001 .$.\n";
        let mut reader = Cursor::new(input);
        let grid = parse_anfield(&mut reader).unwrap();
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.rows, 2);
        assert_eq!(grid.data[0], vec![Cell::Empty, Cell::Player1Old, Cell::Empty]);
        assert_eq!(grid.data[1], vec![Cell::Empty, Cell::Player2Old, Cell::Empty]);
    }

    #[test]
    fn test_parse_piece() {
        let input = "Piece 2 2:\n.#\n#.\n";
        let mut reader = Cursor::new(input);
        let piece = parse_piece(&mut reader).unwrap();
        assert_eq!(piece.cols, 2);
        assert_eq!(piece.rows, 2);
        assert_eq!(piece.blocks, vec![(0, 1), (1, 0)]);
    }
}
