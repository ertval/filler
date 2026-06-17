use filler::visualizer;
use std::io;
use std::time::Duration;

fn main() {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());

    println!("Parsing game replay from stdin...");
    match visualizer::read_replay(&mut reader) {
        Ok(frames) => {
            if frames.is_empty() {
                println!("No game frames found in input.");
                return;
            }
            println!("Playing {} frames...", frames.len());
            // Small pause before rendering
            std::thread::sleep(Duration::from_millis(500));
            visualizer::play(&frames, Duration::from_millis(150));
            println!("\nReplay finished!");
        }
        Err(e) => {
            eprintln!("Error parsing replay: {}", e);
            std::process::exit(1);
        }
    }
}
