# Implementation Methodology

This project was developed by the Antigravity AI agent utilizing a modular, test-driven approach.

## Multi-Agent Architecture

To execute the phases cleanly and in parallel:
- **Main Agent**: Coordinated the plan, implemented Phase 1 (Cargo & types), Phase 2 (Parser), Phase 3 (Validator), Phase 4 (Strategy), Phase 5 (Main loop), and Phase 6 (Integration tests).
- **Subagent Phase 7 (`phase7_implementer`)**: Implemented the `.dockerignore`, `Dockerfile`, the audit execution runner (`run_audit_suite.sh`), and the winrate calculation binary (`assert_winrate.rs`).
- **Subagent Phase 8 (`phase8_implementer`)**: Implemented the bonus visualizer logic (`visualizer.rs`) and its corresponding unit tests.

## Development Principles

1. **Simplicity First**: Followed Karpathy's simplicity rules—minimized external dependencies, adhering strictly to standard library tools.
2. **Surgical Verification**: Wrote inline unit tests and integration tests for every module (totaling 22 tests).
3. **Lexicographical Tie-breaking**: Implemented a robust `(new_dist, own_dist)` tuple score sorting mechanism to resolve flat heatmap scores (especially for 1x1 pieces) deterministically.
4. **Git Operations**: Commits were generated per-ticket/phase using Conventional Commits format and pushed to remote targets (`origin`, `github`).
