.PHONY: build test-lib test test-e2e bench audit docker-build docker-run help run play

# Parameters for running games
MAP ?= map00
P1 ?= filler
P2 ?= bender
VIS ?= 0

build:
	cargo build --release

play: build
	./run_game.sh "$(MAP)" "$(P1)" "$(P2)" "$(VIS)"

run:
	@echo "Run inside game engine:"
	@echo '  ./game_engine -f maps/map01 -p1 ./target/release/filler -p2 robots/bender'

test-lib:
	cargo test --lib

test:
	cargo test

test-e2e:
	cargo test --features e2e --test e2e

bench:
	cargo bench

audit:
	bash e2e/run_audit_suite.sh

docker-build:
	docker build -t filler .

docker-run:
	docker run -v "$(PWD)/solution":/filler/solution -it filler

help:
	@echo "Targets:"
	@echo "  build         Release build (LTO, single codegen)"
	@echo "  run           Show game-engine invocation"
	@echo "  play          Run a game (MAP, P1, P2, VIS)"
	@echo "  test-lib      Unit tests only"
	@echo "  test          All unit + integration tests"
	@echo "  test-e2e      E2E replay validation (needs game_engine)"
	@echo "  bench         Performance benchmark (<500ms assert)"
	@echo "  audit         Full audit suite"
	@echo "  docker-build  Multi-stage Docker build"
	@echo "  docker-run    Run inside container"
