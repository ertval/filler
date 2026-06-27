#!/usr/bin/env bash
set -euo pipefail

# Simple Q1 check: verifies Docker image builds and binary is present
# Usage: make q1-s

echo "[Q1-SIMPLE] Verifying filler Docker image and binary..."

# Reuse docker-build target logic
docker build -t filler . > /dev/null 2>&1

if docker run --rm filler test -f /filler/solution/filler; then
    echo "[PASS] container runs, binary present"
else
    echo "[FAIL] container check failed"
    exit 1
fi