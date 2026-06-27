use crate::types::{Cell, GameState, Grid, Piece, Player};
use std::io::BufRead;

// Audit Q8: Input Parsing — reading Anfield dimensions + piece shape from stdin
// Audit Q2: Project runs without crash — parser errors caught in main loop, no panics

/// Helper — Read a line. Returns error containing "EOF" on end of file.
fn read_line<R: BufRead>(reader: &mut R) -> Result<String, String> {
    loop {
        let mut buf = String::new();
        let bytes_read = reader.read_line(&mut buf).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            return Err("EOF".to_string());
        }
        let trimmed = buf.trim_end_matches('\n').trim_end_matches('\r');
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
}

/// Parse player ID line (TDD)
pub fn parse_player_line(line: &str) -> Result<Player, String> {
    if !line.starts_with("$$$ exec p")
        || line.len() < 11
        || !line.contains(" : [")
        || !line.ends_with(']')
    {
        return Err(format!("invalid player line: {line}"));
    }
    match line.as_bytes()[10] {
        b'1' => Ok(Player::P1),
        b'2' => Ok(Player::P2),
        _ => Err(format!("unknown player in line: {line}")),
    }
}

fn parse_anfield_header(line: &str) -> Result<(usize, usize), String> {
    // "Anfield 20 15:"
    let line = line
        .strip_prefix("Anfield ")
        .ok_or("missing Anfield prefix")?;
    let line = line.strip_suffix(':').ok_or("missing colon")?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("expected 'cols rows' in header".into());
    }
    let cols: usize = parts[0].parse().map_err(|_| "invalid cols")?;
    let rows: usize = parts[1].parse().map_err(|_| "invalid rows")?;
    if cols == 0 || rows == 0 {
        return Err("dimensions must be greater than zero".into());
    }
    Ok((cols, rows))
}

/// Parse Anfield grid header & rows
pub fn parse_anfield<R: BufRead>(reader: &mut R) -> Result<Grid, String> {
    let header = read_line(reader)?;
    let (cols, rows) = parse_anfield_header(&header)?;

    // Skip column header line (e.g. "    012345...")
    let col_header = read_line(reader)?;
    if col_header.trim_start().len() < cols {
        return Err(format!(
            "column header line too short: expected at least {cols} characters, got {}",
            col_header.trim_start().len()
        ));
    }

    let mut data = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = read_line(reader)?;
        let start_idx = line.find(' ').ok_or("missing space after row number")? + 1;
        if line.len() < start_idx + cols {
            return Err(format!(
                "row line too short: expected {}, got {}",
                start_idx + cols,
                line.len()
            ));
        }
        let row_chars = &line[start_idx..start_idx + cols];
        let row: Vec<Cell> = row_chars
            .chars()
            .map(Cell::try_from)
            .collect::<Result<Vec<Cell>, String>>()?;
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
    if cols == 0 || rows == 0 {
        return Err("piece dimensions must be greater than zero".into());
    }
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
            if c >= cols {
                break;
            }
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

    Ok(GameState {
        me,
        opponent,
        grid,
        piece,
    })
}
