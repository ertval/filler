use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut map = String::new();
    let mut p1 = String::new();
    let mut p2 = String::new();
    let mut runs = 5;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--map" => {
                if i + 1 < args.len() {
                    map = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --map requires a value");
                    std::process::exit(1);
                }
            }
            "--p1" => {
                if i + 1 < args.len() {
                    p1 = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --p1 requires a value");
                    std::process::exit(1);
                }
            }
            "--p2" => {
                if i + 1 < args.len() {
                    p2 = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --p2 requires a value");
                    std::process::exit(1);
                }
            }
            "--runs" => {
                if i + 1 < args.len() {
                    if let Ok(r) = args[i + 1].parse::<usize>() {
                        runs = r;
                    } else {
                        eprintln!("Error: --runs requires an integer value");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --runs requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                i += 1;
            }
        }
    }

    if map.is_empty() || p1.is_empty() || p2.is_empty() {
        eprintln!("Usage: assert_winrate --map <map_path> --p1 <player_1> --p2 <player_2> [--runs <num_runs>]");
        std::process::exit(1);
    }

    println!("Running {} games on {}...", runs, map);
    println!("Player 1: {}", p1);
    println!("Player 2: {}", p2);

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for game in 1..=runs {
        println!("Game {}/{}...", game, runs);
        let output = Command::new("./game_engine")
            .arg("-f")
            .arg(&map)
            .arg("-p1")
            .arg(&p1)
            .arg("-p2")
            .arg(&p2)
            .output();

        match output {
            Ok(out) => {
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                let stderr_str = String::from_utf8_lossy(&out.stderr);
                let combined_output = format!("{}\n{}", stdout_str, stderr_str);

                if combined_output.contains("Player 1 won")
                    || combined_output.contains("Player 1 won!")
                {
                    p1_wins += 1;
                    println!("  Winner: Player 1");
                } else if combined_output.contains("Player 2 won")
                    || combined_output.contains("Player 2 won!")
                {
                    p2_wins += 1;
                    println!("  Winner: Player 2");
                } else {
                    println!("  Winner: Unknown / Draw / Crash");
                }
            }
            Err(e) => {
                eprintln!("Failed to execute game_engine: {}", e);
            }
        }
    }

    // Determine which player is the student.
    // If one of the player paths contains "filler" or "solution", assume it's the student.
    // Default to Player 1 if unclear.
    let student_is_p1 = if p1.contains("filler") || p1.contains("solution") {
        true
    } else if p2.contains("filler") || p2.contains("solution") {
        false
    } else {
        true
    };

    let student_wins = if student_is_p1 { p1_wins } else { p2_wins };

    println!("\nResults:");
    println!("Player 1 wins: {}", p1_wins);
    println!("Player 2 wins: {}", p2_wins);
    println!("WINS={}", student_wins);
}
