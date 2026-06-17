# Filler Rust Implementation Tracker

Use this checklist to implement `filler` in Rust. You do not need to read the entire `deeppro.md`; use the references to go directly to the specifications & code blocks.

## 🛠️ Implementation Tasks

### Phase 1: Cargo & Types Setup
- [x] **T1.1: Cargo Config** - Configure release profile & features | [Cargo.toml](file:///home/ertval/code/zone-modules/filler/Cargo.toml) | [deeppro.md:901-927](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L901-L927)
- [x] **T1.2: Core Types** - Define Player, Cell, Grid, Point, Piece, GameState | [src/types.rs](file:///home/ertval/code/zone-modules/filler/src/types.rs) | [deeppro.md:62-172](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L62-L172)
- [x] **T1.3: Lib Re-exports** - Export modules for tests | [src/lib.rs](file:///home/ertval/code/zone-modules/filler/src/lib.rs) | [deeppro.md:888-899](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L888-L899)
- [x] **T1.4: Output Formatter** - Output `X Y` coordinates (TDD) | [src/output.rs](file:///home/ertval/code/zone-modules/filler/src/output.rs) | [deeppro.md:784-807](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L784-L807)


### Phase 2: Stdin Parser (Module A)
- [x] **T2.1: Player Line** - Parse player ID line (TDD) | [src/parser.rs](file:///home/ertval/code/zone-modules/filler/src/parser.rs) | [deeppro.md:221-251](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L221-L251)
- [x] **T2.2: Anfield Grid** - Parse Anfield grid header & rows | [src/parser.rs](file:///home/ertval/code/zone-modules/filler/src/parser.rs) | [deeppro.md:253-307](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L253-L307)
- [x] **T2.3: Piece Parser** - Parse random piece header & blocks | [src/parser.rs](file:///home/ertval/code/zone-modules/filler/src/parser.rs) | [deeppro.md:309-358](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L309-L358)
- [x] **T2.4: Parser Integration** - Full turn parser integration | [src/parser.rs](file:///home/ertval/code/zone-modules/filler/src/parser.rs) | [deeppro.md:360-415](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L360-L415)


### Phase 3: Legality Validator (Module B)
- [x] **T3.1: Bounds Check** - Verify piece fits within grid boundary (TDD) | [src/validator.rs](file:///home/ertval/code/zone-modules/filler/src/validator.rs) | [deeppro.md:431-461](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L431-L461)
- [x] **T3.2: Overlap Validator** - Assert 1 own cell and 0 opponent cells overlap | [src/validator.rs](file:///home/ertval/code/zone-modules/filler/src/validator.rs) | [deeppro.md:463-529](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L463-L529)
- [x] **T3.3: Move Generator** - Scan grid (including negative padding) | [src/validator.rs](file:///home/ertval/code/zone-modules/filler/src/validator.rs) | [deeppro.md:531-572](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L531-L572)


### Phase 4: Strategy Heatmap & Scorer (Module C)
- [x] **T4.1: BFS Heatmap** - Manhattan distance BFS from opponent | [src/strategy.rs](file:///home/ertval/code/zone-modules/filler/src/strategy.rs) | [deeppro.md:583-641](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L583-L641)
- [x] **T4.2: Placement Scorer** - Compute sum of heatmap cells under piece | [src/strategy.rs](file:///home/ertval/code/zone-modules/filler/src/strategy.rs) | [deeppro.md:643-686](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L643-L686)
- [x] **T4.3: Placement Selector** - Pick min score with deterministic tie-breaker | [src/strategy.rs](file:///home/ertval/code/zone-modules/filler/src/strategy.rs) | [deeppro.md:688-738](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L688-L738)
- [x] **T4.4: Dual Heatmap & Tuning** - Advanced placement selection & tuning | [src/strategy.rs](file:///home/ertval/code/zone-modules/filler/src/strategy.rs) | [deeppro.md:739-781](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L739-L781)


### Phase 5: Main Entry & Loop
- [x] **T5.1: Main Loop** - Buffering IO, no-panic guardrail, clean EOF exit | [src/main.rs](file:///home/ertval/code/zone-modules/filler/src/main.rs) | [deeppro.md:808-886](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L808-L886)


### Phase 6: Integration Tests & Quality Gates
- [ ] **T6.1: Test Helpers & Fixtures** - Define standard 5x5 Anfield and mock stdin | [tests/common/mod.rs](file:///home/ertval/code/zone-modules/filler/tests/common/mod.rs) | [deeppro.md:934-951](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L934-L951)
- [ ] **T6.2: E2E Replay Validation** - Assert move correctness | [tests/e2e.rs](file:///home/ertval/code/zone-modules/filler/tests/e2e.rs) | [deeppro.md:1014-1032](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1014-L1032)
- [ ] **T6.3: Integration Tests** - IT-1 to IT-6 turn pipeline simulation | [tests/integration_tests.rs](file:///home/ertval/code/zone-modules/filler/tests/integration_tests.rs) | [deeppro.md:929-985](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L929-L985)
- [ ] **T6.4: Scorer Tie-breaker Test** - Verify deterministic selection | [tests/strategy_tests.rs](file:///home/ertval/code/zone-modules/filler/tests/strategy_tests.rs) | [deeppro.md:1051-1071](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1051-L1071)
- [ ] **T6.5: Growth & Performance** - Turn performance benchmark (< 500ms) & growth tests | [tests/multi_turn.rs](file:///home/ertval/code/zone-modules/filler/tests/multi_turn.rs) | [deeppro.md:987-1012,1034-1049](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L987-L1012)

### Phase 7: Audit Script & Docker Setup
- [x] **T7.1: Win-rate Checkers** - assert_winrate program & audit shell script | [e2e/assert_winrate.rs](file:///home/ertval/code/zone-modules/filler/e2e/assert_winrate.rs) | [deeppro.md:1073-1251](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1073-L1251)
- [x] **T7.2: Dockerization** - Build Debian Bookworm slim multi-stage image | [Dockerfile](file:///home/ertval/code/zone-modules/filler/Dockerfile) | [deeppro.md:1253-1314](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1253-L1314)

### Phase 8: Visualizer (Bonus)
- [x] **T8.1: Replay Renderer** - CLI ANSI colored game replay visualization | [src/visualizer.rs](file:///home/ertval/code/zone-modules/filler/src/visualizer.rs) | [deeppro.md:1465-1508](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1465-L1508)

---

## 🔍 Edge Case Quick Reference
Check implementations against specific edge cases:
- **E1-E2 (Parsing Error Bounds):** [deeppro.md:1424-1425](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1424-L1425) -> Test in `tests/parser_tests.rs`
- **E3-E6 (Placement Edge Cases):** [deeppro.md:1426-1429](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1426-L1429) -> Test in `tests/validator_tests.rs`
- **E7 (EOF Handlers):** [deeppro.md:1430](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1430) -> Test in `src/main.rs`
- **E8-E11 (Overlap & Boundaries):** [deeppro.md:1431-1434](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1431-L1434) -> Test in `tests/validator_tests.rs`
- **E12-E15 (Performance & IO formats):** [deeppro.md:1435-1438](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md#L1435-L1438)

## 📑 Requirements / Audit Map
- **Image & Container Runs:** Covered by T7.2 | Q1-Q2 in `requirements/audit.md`
- **1-cell Overlap Rule:** Covered by T3.2 | Q3 in `requirements/audit.md`
- **Win Rates $\ge 80\%$ vs bots:** Covered by T7.1 | Q4-Q6, Bonus in `requirements/audit.md`
- **Unit & Integration tests:** Covered by Phase 6 | Q7-Q10 in `requirements/audit.md`
