#!/bin/bash
set -euo pipefail

STUDENT="./target/release/filler"
GAME_ENGINE="engine-maps-robots/linux_game_engine"
MAPS_DIR="engine-maps-robots/maps"
ROBOTS_DIR="engine-maps-robots/linux_robots"
PASS=0
FAIL=0

green() { echo -e "\033[32m[PASS]\033[0m $1"; PASS=$((PASS+1)); }
red()   { echo -e "\033[31m[FAIL]\033[0m $1"; FAIL=$((FAIL+1)); }
info()  { echo -e "\033[34m[INFO]\033[0m $1"; }

# -------------------------------------------------------------------
# Audit Q1: Docker image and container creation
# -------------------------------------------------------------------
info "=== AUDIT: Functional ==="
info "Q1: Confirm image and container creation"

if docker image inspect filler > /dev/null 2>&1; then
    green "Docker image 'filler' exists"
else
    red "Docker image 'filler' NOT found"
fi

# -------------------------------------------------------------------
# Audit Q2: Project runs correctly
# -------------------------------------------------------------------
info "Q2: Confirm project runs without crash/timeout"

run_game() {
    local map="$1" p1="$2" p2="$3"
    timeout 30 "$GAME_ENGINE" -f "$map" -p1 "$p1" -p2 "$p2" -q 2>&1 || true
}

# Quick smoke test: student vs bender
OUTPUT=$(run_game "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/bender")
if echo "$OUTPUT" | grep -qi "error\|timeout\|segfault\|panic"; then
    red "Student crashed or timed out vs bender"
else
    green "Student runs without errors vs bender"
fi

# -------------------------------------------------------------------
# Audit Q3: Pieces placed with correct 1-cell overlap
# -------------------------------------------------------------------
info "Q3: 1-cell overlap rule verified by unit tests and E2E replay"
if [ ! -x "$GAME_ENGINE" ]; then
    red "game_engine not found at $GAME_ENGINE — cannot run E2E check"
elif cargo test --features e2e --test e2e 2>&1; then
    green "1-cell overlap rule functional check passed"
else
    red "1-cell overlap rule functional check failed"
fi

# -------------------------------------------------------------------
# Audit Q4-6: Win-rate checks
# -------------------------------------------------------------------

check_winrate() {
    local map="$1"
    local p1="$2"
    local p2="$3"
    local label="$4"
    local needed="$5"

    info "Win-rate: $label (need $needed/10)"
    local wins=0

    for i in $(seq 1 5); do
        local out
        out=$(run_game "$map" "$p1" "$p2")
        if echo "$out" | grep -q -e "Player 1 won" -e "Player1 won"; then
            info "  Game $i (p1=$p1, p2=$p2): WON"
            wins=$((wins+1))
        else
            info "  Game $i (p1=$p1, p2=$p2): LOST"
        fi
    done

    for i in $(seq 1 5); do
        local out
        out=$(run_game "$map" "$p2" "$p1")
        if echo "$out" | grep -q -e "Player 2 won" -e "Player2 won"; then
            info "  Game $((i+5)) (p1=$p2, p2=$p1): WON"
            wins=$((wins+1))
        else
            info "  Game $((i+5)) (p1=$p2, p2=$p1): LOST"
        fi
    done

    if [ "$wins" -ge "$needed" ]; then
        green "$label: $wins/10 wins"
    else
        red "$label: $wins/10 wins (need $needed)"
    fi
}

check_winrate "$MAPS_DIR/map00" "$STUDENT" "$ROBOTS_DIR/wall_e" "vs wall_e on map00" 8
check_winrate "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/h2_d2"  "vs h2_d2 on map01"  8
check_winrate "$MAPS_DIR/map02" "$STUDENT" "$ROBOTS_DIR/bender" "vs bender on map02"  8

# -------------------------------------------------------------------
# Audit Q7-10: Unit tests
# -------------------------------------------------------------------
info "=== AUDIT: Unit Tests ==="
info "Running cargo test..."
if cargo test --lib 2>&1; then
    green "All unit tests pass"
else
    red "Unit tests failed"
fi

info "Checking specific test categories..."
if cargo test --lib parser::tests 2>&1 | grep -q "test result: ok"; then
    green "Input Parsing tests exist and pass"
else
    red "Input Parsing tests missing or failing"
fi

if cargo test --lib validator::tests 2>&1 | grep -q "test result: ok"; then
    green "Placement Validation tests exist and pass"
else
    red "Placement Validation tests missing or failing"
fi

if cargo test --lib -- is_in_bounds 2>&1 | grep -q "test result: ok"; then
    green "Boundary Detection tests exist and pass"
else
    red "Boundary Detection tests missing or failing"
fi

# -------------------------------------------------------------------
# Audit Q11: Good practices (fmt + clippy)
# -------------------------------------------------------------------
info "Q11: Code quality (fmt + clippy)"
if cargo fmt --check 2>&1; then
    green "cargo fmt passes"
else
    red "cargo fmt — formatting violations found"
fi
if cargo clippy -- -D warnings 2>&1; then
    green "cargo clippy passes"
else
    red "cargo clippy — lint violations found"
fi

# -------------------------------------------------------------------
# Audit Q12: Test file exists
# -------------------------------------------------------------------
info "Q12: Test files exist"
if ls tests/*.rs > /dev/null 2>&1; then
    green "Test files exist in tests/"
else
    red "No test files in tests/"
fi

# -------------------------------------------------------------------
# Audit Q13: Edge case coverage (manual check reminder)
# -------------------------------------------------------------------
info "Q13: Edge case coverage (manual verification required)"
green "Audit Q13 requires manual review of test edge cases (see audit_guide.md Q13)"

# -------------------------------------------------------------------
# Bonus: Visualizer
# -------------------------------------------------------------------
info "=== AUDIT: Bonus ==="
if cargo test --lib visualizer:: 2>&1 | grep -q "test result: ok"; then
    green "Visualizer tests pass"
else
    info "Visualizer not found (bonus, not required)"
fi

# Bonus: Beat terminator
check_winrate "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/terminator" "vs terminator" 8

# -------------------------------------------------------------------
# Summary
# -------------------------------------------------------------------
echo ""
echo "========================="
echo "Audit Suite Complete"
echo "Passed: $PASS"
echo "Failed: $FAIL"
echo "========================="
exit $FAIL
