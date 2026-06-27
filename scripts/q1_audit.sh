#!/usr/bin/env bash
set -euo pipefail

# Full Q1 Audit: Build zone-filler + filler images, mount student binary, run zone01 audit inside container
# Usage: make q1

echo "[STEP 1] Building zone-filler image..."
docker build -t zone-filler engine-maps-robots/ > /dev/null 2>&1

echo "[STEP 2] Building filler image..."
docker build -t filler . > /dev/null 2>&1

echo "[STEP 3] Running zone-filler container with student filler mounted..."
docker run -it --rm \
  -v "$PWD/target/release/filler:/filler/student_filler" \
  zone-filler sh -c '
    echo "[STEP 4a] Running zone01 audit: bender vs terminator..."
    ./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator

    echo ""
    echo "[STEP 4b] Running student as P1 vs bender..."
    ./linux_game_engine -f maps/map01 -p1 ./student_filler -p2 linux_robots/bender
  '

echo "[DONE] Q1 audit complete."