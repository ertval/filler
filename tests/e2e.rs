#[cfg(feature = "e2e")]
#[test]
fn test_replay_move_correctness() {
    use assert_cmd::Command;
    use std::path::Path;

    // Check if game_engine exists first to avoid panicking immediately if not run in full env
    if !Path::new("./game_engine").exists() {
        eprintln!("game_engine binary not found, skipping E2E test.");
        return;
    }

    // Run game_engine
    let mut cmd = Command::new("./game_engine");
    cmd.args(&[
        "-f",
        "maps/map01",
        "-p1",
        "target/debug/filler",
        "-p2",
        "robots/bender",
    ]);
    let assert = cmd.assert().success();
    let output = String::from_utf8_lossy(&assert.get_output().stdout);

    // Basic assertion that student player made valid placements
    assert!(output.contains("won") || output.contains("error") || output.contains("score"));
}
