use std::io::{self, Write, BufWriter};
use filler::parser;
use filler::validator;
use filler::strategy;
use filler::output;
use filler::types::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    // Wrap stdout in BufWriter for efficient buffered writes
    let mut writer = BufWriter::new(stdout.lock());

    let mut state: Option<GameState> = None;

    loop {
        let turn = match parser::parse_turn(&mut reader, state.clone()) {
            Ok(t) => t,
            Err(e) => {
                // Exit cleanly on EOF
                if e.contains("EOF") || e.contains("unexpected end of file") {
                    break;
                }
                eprintln!("Parse error: {e}");
                writer.write_all(output::format_no_move().as_bytes())?;
                writer.flush()?;
                continue;
            }
        };

        let valid = validator::find_valid_placements(
            &turn.grid,
            &turn.piece,
            turn.me,
        );

        if valid.is_empty() {
            writer.write_all(output::format_no_move().as_bytes())?;
            writer.flush()?;
            state = Some(turn);
            continue;
        }

        let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
        let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece);

        match chosen {
            Some(p) => writer.write_all(output::format_move(p).as_bytes())?,
            None     => writer.write_all(output::format_no_move().as_bytes())?,
        }
        writer.flush()?;

        state = Some(turn);
    }

    Ok(())
}
