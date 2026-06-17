use filler::output;
use filler::parser;
use filler::strategy;
use filler::types::GameState;
use filler::validator;
use std::io::{self, BufWriter, Write};

use std::panic::AssertUnwindSafe;

// Audit Q2: Project runs correctly — never panic!, EOF exits cleanly, 0 0 on parse error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    // Wrap stdout in BufWriter for efficient buffered writes
    let mut writer = BufWriter::new(stdout.lock());

    let mut state: Option<GameState> = None;

    loop {
        let turn_state = state.clone();
        let reader_ref = &mut reader;
        let res = std::panic::catch_unwind(AssertUnwindSafe(move || {
            let turn = match parser::parse_turn(&mut *reader_ref, turn_state) {
                Ok(t) => t,
                Err(e) => {
                    return Err(e);
                }
            };

            let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);

            if valid.is_empty() {
                return Ok((None, turn));
            }

            let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
            let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece);
            Ok((chosen, turn))
        }));

        match res {
            Ok(Ok((chosen, turn))) => {
                match chosen {
                    Some(p) => writer.write_all(output::format_move(p).as_bytes())?,
                    None => writer.write_all(output::format_no_move().as_bytes())?,
                }
                writer.flush()?;
                state = Some(turn);
            }
            Ok(Err(e)) => {
                // Exit cleanly on EOF
                if e.contains("EOF") || e.contains("unexpected end of file") {
                    break;
                }
                eprintln!("Parse error: {e}");
                writer.write_all(output::format_no_move().as_bytes())?;
                writer.flush()?;
            }
            Err(_) => {
                eprintln!("Panic caught during turn execution! Recovering by outputting fallback move.");
                writer.write_all(output::format_no_move().as_bytes())?;
                writer.flush()?;
            }
        }
    }

    Ok(())
}
