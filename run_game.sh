#!/bin/bash
set -e

# Default parameters
MAP="${1:-map00}"
P1="${2:-filler}"
P2="${3:-bender}"
VIS="${4:-0}"

# Detect OS/architecture for game engine
UNAME_S=$(uname -s)
UNAME_M=$(uname -m)

if [ "$UNAME_S" = "Darwin" ] && [ "$UNAME_M" = "arm64" ]; then
    ENGINE_DIR="m1"
else
    ENGINE_DIR="linux"
fi

ENGINE_PREFIX="engine-maps-robots/$ENGINE_DIR"
ENGINE="${ENGINE_PREFIX}_game_engine"
ROBOTS_DIR="${ENGINE_PREFIX}_robots"

# Resolve Map Path
if [ -f "$MAP" ]; then
    MAP_PATH="$MAP"
elif [ -f "engine-maps-robots/maps/$MAP" ]; then
    MAP_PATH="engine-maps-robots/maps/$MAP"
else
    echo "Error: Map not found: $MAP" >&2
    exit 1
fi

# Resolve Player 1 Path
if [ "$P1" = "filler" ]; then
    P1_PATH="./target/release/filler"
elif [ -f "$P1" ]; then
    P1_PATH="$P1"
elif [ -f "$ROBOTS_DIR/$P1" ]; then
    P1_PATH="$ROBOTS_DIR/$P1"
else
    echo "Error: Player 1 not found: $P1" >&2
    exit 1
fi

# Resolve Player 2 Path
if [ "$P2" = "filler" ]; then
    P2_PATH="./target/release/filler"
elif [ -f "$P2" ]; then
    P2_PATH="$P2"
elif [ -f "$ROBOTS_DIR/$P2" ]; then
    P2_PATH="$ROBOTS_DIR/$P2"
else
    echo "Error: Player 2 not found: $P2" >&2
    exit 1
fi

# Ensure engine is executable
chmod +x "$ENGINE"

# Execute
if [ "$VIS" = "1" ] || [ "$VIS" = "true" ] || [ "$VIS" = "yes" ]; then
    "$ENGINE" -f "$MAP_PATH" -p1 "$P1_PATH" -p2 "$P2_PATH" | ./target/release/visualizer
else
    "$ENGINE" -f "$MAP_PATH" -p1 "$P1_PATH" -p2 "$P2_PATH"
fi
