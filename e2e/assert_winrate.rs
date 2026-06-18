use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut map = String::new();
    let mut p1 = String::new();
    let mut p2 = String::new();
    let mut runs = 5;
    let mut engine = "./game_engine".to_string();

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
            "--engine" => {
                if i + 1 < args.len() {
                    engine = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --engine requires a value");
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
        eprintln!("Usage: assert_winrate --map <map_path> --p1 <player_1> --p2 <player_2> [--runs <num_runs>] [--engine <engine_path>]");
        std::process::exit(1);
    }

    // Identify student and opponent.
    let (student_path, bot_path) = if p1.contains("filler") || p1.contains("solution") {
        (p1.clone(), p2.clone())
    } else if p2.contains("filler") || p2.contains("solution") {
        (p2.clone(), p1.clone())
    } else {
        (p1.clone(), p2.clone())
    };

    println!("Running {} games on {} using engine {}...", runs, map, engine);
    println!("Student: {}", student_path);
    println!("Bot: {}", bot_path);

    let mut student_wins = 0;
    let mut bot_wins = 0;

    for game in 1..=runs {
        // Swap roles: student is P1 on odd games, P2 on even games
        let student_is_p1 = game % 2 != 0;
        let (p1_arg, p2_arg) = if student_is_p1 {
            (&student_path, &bot_path)
        } else {
            (&bot_path, &student_path)
        };

        println!("Game {}/{} (Student is Player {})...", game, runs, if student_is_p1 { 1 } else { 2 });
        let output = Command::new(&engine)
            .arg("-f")
            .arg(&map)
            .arg("-p1")
            .arg(p1_arg)
            .arg("-p2")
            .arg(p2_arg)
            .arg("-q") // Run in quiet mode
            .output();

        match output {
            Ok(out) => {
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                let stderr_str = String::from_utf8_lossy(&out.stderr);
                let combined_output = format!("{}\n{}", stdout_str, stderr_str);

                let p1_won = combined_output.contains("Player 1 won")
                    || combined_output.contains("Player 1 won!")
                    || combined_output.contains("Player1 won")
                    || combined_output.contains("Player1 won!");
                let p2_won = combined_output.contains("Player 2 won")
                    || combined_output.contains("Player 2 won!")
                    || combined_output.contains("Player2 won")
                    || combined_output.contains("Player2 won!");

                if p1_won {
                    if student_is_p1 {
                        student_wins += 1;
                        println!("  Winner: Student (Player 1)");
                    } else {
                        bot_wins += 1;
                        println!("  Winner: Bot (Player 1)");
                    }
                } else if p2_won {
                    if !student_is_p1 {
                        student_wins += 1;
                        println!("  Winner: Student (Player 2)");
                    } else {
                        bot_wins += 1;
                        println!("  Winner: Bot (Player 2)");
                    }
                } else {
                    println!("  Winner: Unknown / Draw / Crash");
                }
            }
            Err(e) => {
                eprintln!("Failed to execute game_engine ({}): {}", engine, e);
            }
        }
    }

    println!("\nResults:");
    println!("Student wins: {}", student_wins);
    println!("Bot wins: {}", bot_wins);
    println!("WINS={}", student_wins);

    let win_rate = student_wins as f64 / runs as f64;
    if win_rate < 0.8 {
        eprintln!("Error: Student win rate is {:.1}% ({} wins out of {} runs), below the 80% requirement!", win_rate * 100.0, student_wins, runs);
        std::process::exit(1);
    } else {
        println!("Success: Student win rate is {:.1}% ({} wins out of {} runs), meeting the 80% requirement.", win_rate * 100.0, student_wins, runs);
    }
}
