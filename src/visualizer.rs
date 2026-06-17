use crate::types::{Cell, Grid};
use std::io::{self, BufRead, Write};
use std::time::Duration;

// Audit Bonus: Visualizer — CLI ANSI colored game replay

/// One frame = one game turn's grid state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub grid: Grid,
    pub turn: usize,
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
    Ok((cols, rows))
}

/// Parse a game replay from a reader
pub fn read_replay<R: BufRead>(reader: &mut R) -> Result<Vec<Frame>, String> {
    let mut frames = Vec::new();
    let mut turn = 0;

    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
        if trimmed.starts_with("Anfield ") {
            let (cols, rows) = parse_anfield_header(trimmed)?;

            // Skip the next line, which is the column header (e.g. "    012345...")
            let mut col_header = String::new();
            let header_bytes = reader
                .read_line(&mut col_header)
                .map_err(|e| e.to_string())?;
            if header_bytes == 0 {
                return Err("Unexpected EOF after Anfield header (missing column index)".into());
            }

            // Read rows
            let mut data = Vec::with_capacity(rows);
            for _ in 0..rows {
                let mut row_line = String::new();
                let bytes = reader.read_line(&mut row_line).map_err(|e| e.to_string())?;
                if bytes == 0 {
                    return Err("Unexpected EOF while reading grid rows".into());
                }
                let trimmed_row = row_line.trim_end_matches('\n').trim_end_matches('\r');
                if trimmed_row.len() < 4 + cols {
                    return Err(format!(
                        "row line too short: expected {}, got {}",
                        4 + cols,
                        trimmed_row.len()
                    ));
                }
                let row_chars = &trimmed_row[4..4 + cols];
                let row: Vec<Cell> = row_chars.chars().map(Cell::from_char).collect();
                data.push(row);
            }

            let grid = Grid { rows, cols, data };
            frames.push(Frame { grid, turn });
            turn += 1;
        }
    }

    Ok(frames)
}

/// Render a single frame with ANSI colors
pub fn render_frame(frame: &Frame, writer: &mut impl Write) -> io::Result<()> {
    // Clear screen: \x1b[2J\x1b[H
    write!(writer, "\x1b[2J\x1b[H")?;
    writeln!(writer, "Turn: {}", frame.turn)?;

    // Print grid row by row
    for row in &frame.grid.data {
        let mut row_str = String::new();
        for cell in row {
            match cell {
                Cell::Empty => row_str.push_str("\x1b[90m.\x1b[0m"),
                Cell::Player1Old => row_str.push_str("\x1b[31m@\x1b[0m"),
                Cell::Player1Recent => row_str.push_str("\x1b[31ma\x1b[0m"),
                Cell::Player2Old => row_str.push_str("\x1b[34m$\x1b[0m"),
                Cell::Player2Recent => row_str.push_str("\x1b[34ms\x1b[0m"),
            }
        }
        writeln!(writer, "{}", row_str)?;
    }
    Ok(())
}

/// Play all frames with delay
pub fn play(frames: &[Frame], delay: Duration) {
    for frame in frames {
        if let Err(e) = render_frame(frame, &mut io::stdout()) {
            eprintln!("Failed to render frame: {}", e);
            break;
        }
        let _ = io::stdout().flush();
        std::thread::sleep(delay);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_single_frame() {
        let input = concat!("Anfield 3 2:\n", "    012\n", "000 .@.\n", "001 .$.\n");
        let mut reader = Cursor::new(input);
        let frames = read_replay(&mut reader).unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].turn, 0);
        assert_eq!(frames[0].grid.cols, 3);
        assert_eq!(frames[0].grid.rows, 2);
        assert_eq!(
            frames[0].grid.data[0],
            vec![Cell::Empty, Cell::Player1Old, Cell::Empty]
        );
        assert_eq!(
            frames[0].grid.data[1],
            vec![Cell::Empty, Cell::Player2Old, Cell::Empty]
        );
    }

    #[test]
    fn test_parse_multiple_frames_with_noise() {
        let input = concat!(
            "$$$ exec p1 : [robots/bender]\n",
            "Anfield 3 2:\n",
            "    012\n",
            "000 .@.\n",
            "001 .$.\n",
            "Piece 1 1:\n",
            "O\n",
            "Anfield 3 2:\n",
            "    012\n",
            "000 .a.\n",
            "001 .s.\n",
            "some extra engine log line\n"
        );
        let mut reader = Cursor::new(input);
        let frames = read_replay(&mut reader).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].turn, 0);
        assert_eq!(frames[1].turn, 1);
        assert_eq!(
            frames[0].grid.data[0],
            vec![Cell::Empty, Cell::Player1Old, Cell::Empty]
        );
        assert_eq!(
            frames[1].grid.data[0],
            vec![Cell::Empty, Cell::Player1Recent, Cell::Empty]
        );
        assert_eq!(
            frames[1].grid.data[1],
            vec![Cell::Empty, Cell::Player2Recent, Cell::Empty]
        );
    }

    #[test]
    fn test_parse_invalid_grid_handling() {
        // Missing row data
        let input = concat!("Anfield 3 2:\n", "    012\n", "000 .@.\n");
        let mut reader = Cursor::new(input);
        assert!(read_replay(&mut reader).is_err());

        // Row line too short
        let input = concat!("Anfield 3 2:\n", "    012\n", "000 .@.\n", "001 .$\n");
        let mut reader = Cursor::new(input);
        assert!(read_replay(&mut reader).is_err());
    }

    #[test]
    fn test_render_frame_output() {
        let grid = Grid {
            rows: 2,
            cols: 3,
            data: vec![
                vec![Cell::Empty, Cell::Player1Old, Cell::Player1Recent],
                vec![Cell::Player2Old, Cell::Player2Recent, Cell::Empty],
            ],
        };
        let frame = Frame { grid, turn: 5 };
        let mut buf = Vec::new();
        render_frame(&frame, &mut buf).unwrap();
        let rendered = String::from_utf8(buf).unwrap();

        assert!(rendered.contains("\x1b[2J\x1b[H"));
        assert!(rendered.contains("Turn: 5"));
        assert!(rendered.contains("\x1b[90m.\x1b[0m\x1b[31m@\x1b[0m\x1b[31ma\x1b[0m"));
        assert!(rendered.contains("\x1b[34m$\x1b[0m\x1b[34ms\x1b[0m\x1b[90m.\x1b[0m"));
    }
}
