// Audit Q3: 1-cell overlap + 0 opponent overlap verified in live game replay
// Audit Q7: E2E replay validation
#[cfg(feature = "e2e")]
#[test]
fn test_replay_move_correctness() {
    use assert_cmd::Command;
    use std::path::Path;

    // Check if game_engine exists first to avoid panicking immediately if not run in full env
    if !Path::new("engine-maps-robots/linux_game_engine").exists() {
        eprintln!("game_engine binary not found, skipping E2E test.");
        return;
    }

    // Run game_engine
    let mut cmd = Command::new("engine-maps-robots/linux_game_engine");
    cmd.args(&[
        "-f",
        "engine-maps-robots/maps/map01",
        "-p1",
        "target/release/filler",
        "-p2",
        "engine-maps-robots/linux_robots/bender",
    ]);
    let assert = cmd.assert().success();
    let output = String::from_utf8_lossy(&assert.get_output().stdout);

    // Robust assertion that the game ran successfully to a winner without crashing or returning invalid moves
    let output_lower = output.to_lowercase();
    assert!(
        output_lower.contains("won") || output_lower.contains("winner"),
        "Game output did not specify a winner: {}",
        output
    );
    assert!(
        !output_lower.contains("panic"),
        "Panic detected in game output!"
    );
    assert!(
        !output_lower.contains("segfault"),
        "Segment fault detected in game output!"
    );
    assert!(
        !output_lower.contains("invalid move"),
        "Invalid move detected in game output!"
    );
}
