.PHONY: build test-lib test test-e2e bench audit docker-build docker-run help run play
.PHONY: q1 q2 q3 q4 q5 q6 q7 q8 q9 q10 q11 q12 q13
.PHONY: qb-visualizer qb-terminator

# Parameters for running games
MAP ?= map00
P1 ?= terminator
P2 ?= filler
VIS ?= 0

# Paths for audit targets (project root, outside container)
ENGINE = engine-maps-robots/linux_game_engine
ROBOTS = engine-maps-robots/linux_robots
MAPS = engine-maps-robots/maps
STUDENT = ./target/release/filler

build:
	cargo build --release

play: build
	./run_game.sh "$(MAP)" "$(P1)" "$(P2)" "$(VIS)"

run:
	@echo "Run from project root:"
	@echo '  make q2'
	@echo "Or manually:"
	@echo '  engine-maps-robots/linux_game_engine -f engine-maps-robots/maps/map01 -p1 ./target/release/filler -p2 engine-maps-robots/linux_robots/bender'

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
	@mkdir -p solution
	docker run -v "$(CURDIR)/solution":/filler/solution -it filler

# ── Audit per-question targets (Q1–Q13) ──────────────────────────

# Reusable win-rate check: $(1)=map_path, $(2)=robot_path
define winrate_check
	@wins=0; total=10; \
	for i in 1 2 3 4 5; do \
	  out=$$($(ENGINE) -f $(1) -p1 $(STUDENT) -p2 $(2) -q 2>&1); \
	  if echo "$$out" | grep -q -e "Player 1 won" -e "Player1 won"; then wins=$$((wins+1)); fi; \
	done; \
	for i in 1 2 3 4 5; do \
	  out=$$($(ENGINE) -f $(1) -p1 $(2) -p2 $(STUDENT) -q 2>&1); \
	  if echo "$$out" | grep -q -e "Player 2 won" -e "Player2 won"; then wins=$$((wins+1)); fi; \
	done; \
	echo "Wins: $$wins/$$total (need 8)"; \
	if [ $$wins -ge 8 ]; then echo "[PASS]"; else echo "[FAIL]"; exit 1; fi
endef

# Functional
q1: docker-build
	@docker run --rm filler test -f /filler/solution/filler && echo "[PASS] container runs, binary present" || echo "[FAIL] container check failed"

q2: build
	$(ENGINE) -f $(MAPS)/map01 -p1 $(STUDENT) -p2 $(ROBOTS)/bender

q3: test-e2e

q4: build
	$(call winrate_check,$(MAPS)/map00,$(ROBOTS)/wall_e)

q5: build
	$(call winrate_check,$(MAPS)/map01,$(ROBOTS)/h2_d2)

q6: build
	$(call winrate_check,$(MAPS)/map02,$(ROBOTS)/bender)

# Unit Tests
q7:
	cargo test

q8:
	cargo test --test parser_tests

q9:
	cargo test --test validator_tests

q10:
	cargo test is_in_bounds

# Basic (code review — automate what we can, manual for the rest)
q11:
	@cargo fmt --check
	@cargo clippy --deny warnings

q12:
	@ls tests/*.rs > /dev/null 2>&1 && echo "[PASS] test files exist" || echo "[FAIL] no test files"

q13:
	@echo "Manual check: tests/ files cover edge cases (see audit guide Q13)"

# Bonus
qb-visualizer:
	cargo test --lib visualizer::

qb-terminator: build
	$(call winrate_check,$(MAPS)/map01,$(ROBOTS)/terminator)

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
	@echo ""
	@echo "Audit per-question targets:"
	@echo "  q1  Docker image build"
	@echo "  q2  Smoke test (no crash vs bender)"
	@echo "  q3  1-cell overlap rule (e2e replay)"
	@echo "  q4  Win-rate vs wall_e on map00"
	@echo "  q5  Win-rate vs h2_d2 on map01"
	@echo "  q6  Win-rate vs bender on map02"
	@echo "  q7  All unit tests pass"
	@echo "  q8  Input parsing tests"
	@echo "  q9  Placement validation tests"
	@echo "  q10 Boundary detection tests"
	@echo "  q11 Code quality (fmt + clippy)"
	@echo "  q12 Test files exist"
	@echo "  q13 Edge case coverage (manual check)"
	@echo ""
	@echo "Bonus targets:"
	@echo "  qb-visualizer  Visualizer tests"
	@echo "  qb-terminator  Win-rate vs terminator"
