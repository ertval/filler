FROM debian:bookworm-slim

WORKDIR /filler

# Create required directories
RUN mkdir -p maps linux_robots

# Copy student filler binary (must exist in build context)
COPY target/release/filler ./student_filler

# Copy game engine, maps, and reference robots
COPY engine-maps-robots/linux_game_engine ./linux_game_engine
COPY engine-maps-robots/maps ./maps
COPY engine-maps-robots/linux_robots ./linux_robots

# Make binaries executable
RUN chmod +x linux_game_engine student_filler linux_robots/*

# Default to running the audit game: bender vs terminator (Q1 zone01 requirement)
CMD ["./linux_game_engine", "-f", "maps/map01", "-p1", "linux_robots/bender", "-p2", "linux_robots/terminator"]