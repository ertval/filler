# Stage 1: Build
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /filler
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY e2e ./e2e
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /filler
COPY --from=builder /filler/target/release/filler /filler/solution/filler
COPY --from=builder /filler/target/release/assert_winrate /filler/target/release/assert_winrate
COPY --from=builder /filler/e2e/run_audit_suite.sh /filler/e2e/run_audit_suite.sh
# game_engine, maps, robots are provided separately in the docker_image folder

CMD ["/bin/bash"]
