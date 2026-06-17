# Graph Report - filler  (2026-06-18)

## Corpus Check
- 88 files · ~52,038 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 816 nodes · 925 edges · 84 communities (66 shown, 18 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 17 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ca21bbce`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Graphify Core Pipeline|Graphify Core Pipeline]]
- [[_COMMUNITY_Compress Detection Utils|Compress Detection Utils]]
- [[_COMMUNITY_Compress Detection OpenCode|Compress Detection OpenCode]]
- [[_COMMUNITY_Caveman Agent Skills|Caveman Agent Skills]]
- [[_COMMUNITY_Compress CLI Tool|Compress CLI Tool]]
- [[_COMMUNITY_Filler DeepPro Plan|Filler DeepPro Plan]]
- [[_COMMUNITY_File Type Detection Utils|File Type Detection Utils]]
- [[_COMMUNITY_File Type Detection OpenCode|File Type Detection OpenCode]]
- [[_COMMUNITY_Text Compression Logic|Text Compression Logic]]
- [[_COMMUNITY_Benchmark Utils|Benchmark Utils]]
- [[_COMMUNITY_Benchmark OpenCode|Benchmark OpenCode]]
- [[_COMMUNITY_Karpathy Guidelines|Karpathy Guidelines]]
- [[_COMMUNITY_No-Mistakes Pipeline|No-Mistakes Pipeline]]
- [[_COMMUNITY_CLI Entry Points|CLI Entry Points]]
- [[_COMMUNITY_OpenCode Plugin Config|OpenCode Plugin Config]]
- [[_COMMUNITY_Plugin Package Config|Plugin Package Config]]
- [[_COMMUNITY_Graphify Knowledge Graph|Graphify Knowledge Graph]]
- [[_COMMUNITY_Karpathy Rules|Karpathy Rules]]
- [[_COMMUNITY_Package Init Utils|Package Init Utils]]
- [[_COMMUNITY_Package Init OpenCode|Package Init OpenCode]]
- [[_COMMUNITY_Caveman Communication|Caveman Communication]]
- [[_COMMUNITY_Find Skills|Find Skills]]
- [[_COMMUNITY_Gitea Tea CLI|Gitea Tea CLI]]
- [[_COMMUNITY_RTK Token Proxy|RTK Token Proxy]]
- [[_COMMUNITY_Node ID Rules|Node ID Rules]]
- [[_COMMUNITY_Graphify Update Reference|Graphify Update Reference]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 49|Community 49]]
- [[_COMMUNITY_Community 50|Community 50]]
- [[_COMMUNITY_Community 51|Community 51]]
- [[_COMMUNITY_Community 53|Community 53]]
- [[_COMMUNITY_Community 54|Community 54]]
- [[_COMMUNITY_Community 56|Community 56]]
- [[_COMMUNITY_Community 57|Community 57]]
- [[_COMMUNITY_Community 61|Community 61]]
- [[_COMMUNITY_Community 62|Community 62]]
- [[_COMMUNITY_Community 63|Community 63]]
- [[_COMMUNITY_Community 65|Community 65]]
- [[_COMMUNITY_Community 66|Community 66]]
- [[_COMMUNITY_Community 67|Community 67]]
- [[_COMMUNITY_Community 68|Community 68]]
- [[_COMMUNITY_Community 69|Community 69]]
- [[_COMMUNITY_Community 70|Community 70]]
- [[_COMMUNITY_Community 72|Community 72]]
- [[_COMMUNITY_Community 73|Community 73]]
- [[_COMMUNITY_Community 74|Community 74]]
- [[_COMMUNITY_Community 75|Community 75]]
- [[_COMMUNITY_Community 76|Community 76]]
- [[_COMMUNITY_Community 77|Community 77]]
- [[_COMMUNITY_Community 78|Community 78]]
- [[_COMMUNITY_Community 79|Community 79]]
- [[_COMMUNITY_Community 80|Community 80]]
- [[_COMMUNITY_Community 81|Community 81]]
- [[_COMMUNITY_Community 84|Community 84]]
- [[_COMMUNITY_Community 88|Community 88]]
- [[_COMMUNITY_Community 92|Community 92]]

## God Nodes (most connected - your core abstractions)
1. `Graphify Knowledge Graph Tool` - 30 edges
2. `Gitea CLI (tea)` - 20 edges
3. `Filler Implementation Plan — TDD in Rust (DeepPro Edition)` - 19 edges
4. `Caveman Compress` - 12 edges
5. `Caveman Help` - 12 edges
6. `no-mistakes` - 12 edges
7. `caveman` - 11 edges
8. `What You Must Do When Invoked` - 11 edges
9. `What You Must Do When Invoked` - 11 edges
10. `validate()` - 10 edges

## Surprising Connections (you probably didn't know these)
- `Karpathy Guidelines` --semantically_similar_to--> `Karpathy Guidelines`  [INFERRED] [semantically similar]
  AGENTS.md → .agents/rules/karpathy-guidelines.md
- `Graphify Knowledge Graph` --semantically_similar_to--> `graphify`  [INFERRED] [semantically similar]
  AGENTS.md → .agents/rules/graphify.md
- `Caveman Communication Mode` --semantically_similar_to--> `Caveman Communication Mode`  [INFERRED] [semantically similar]
  .github/copilot-instructions.md → .opencode/AGENTS.md
- `Compress` --semantically_similar_to--> `Caveman Compress`  [INFERRED] [semantically similar]
  .agents/skills/compress/SKILL.md → .agents/skills/caveman-compress/SKILL.md
- `Karpathy Guidelines` --conceptually_related_to--> `Graphify Knowledge Graph Tool`  [INFERRED]
  .agents/skills/karpathy-guidelines/SKILL.md → .opencode/skills/graphify/SKILL.md

## Import Cycles
- 1-file cycle: `src/output.rs -> src/output.rs`
- 1-file cycle: `src/visualizer.rs -> src/visualizer.rs`

## Hyperedges (group relationships)
- **Caveman Toolkit Ecosystem** — caveman_skill_caveman, caveman_commit_skill_caveman_commit, caveman_compress_skill_caveman_compress, caveman_help_skill_caveman_help, caveman_review_skill_caveman_review, caveman_stats_skill_caveman_stats [EXTRACTED 1.00]
- **Cavecrew Delegation Pipeline** — cavecrew_skill_cavecrew, cavecrew_skill_investigator, cavecrew_skill_builder, cavecrew_skill_reviewer [EXTRACTED 1.00]
- **Auto-Clarity Safety Protocol** — caveman_skill_auto_clarity, caveman_skill_caveman, caveman_commit_skill_caveman_commit, caveman_review_skill_caveman_review [EXTRACTED 1.00]
- **Graphify Core Pipeline Components** — graphify_skill_ast_extraction, graphify_skill_semantic_extraction, graphify_skill_community_detection, graphify_skill_god_nodes, graphify_skill_extraction_cache [EXTRACTED 1.00]
- **Karpathy Four Principles** — karpathy_guidelines_skill_think_before_coding, karpathy_guidelines_skill_simplicity_first, karpathy_guidelines_skill_surgical_changes, karpathy_guidelines_skill_goal_driven_execution [EXTRACTED 1.00]
- **No-Mistakes Pipeline Components** — no_mistakes_skill_axi_command, no_mistakes_skill_gate_system, no_mistakes_skill_intent_requirement, no_mistakes_skill_toon_format [EXTRACTED 1.00]
- **Cross-Plan Enrichment Analysis** — report_cross_plan_enrichments, report_final_cross_plan_enrichments, report_gem_report, plan_deeppro [EXTRACTED 1.00]
- **Karpathy Guidelines Principles** — karpathy_guidelines_think_before_coding, karpathy_guidelines_simplicity_first, karpathy_guidelines_surgical_changes, karpathy_guidelines_goal_driven_execution, karpathy_guidelines_skill [EXTRACTED 1.00]
- **Filler Implementation Artifacts** — plan_deeppro, plan_implementation_tracker, requirements_audit, requirements_requirements [EXTRACTED 1.00]

## Communities (84 total, 18 thin omitted)

### Community 0 - "Graphify Core Pipeline"
Cohesion: 0.06
Nodes (36): AST Extraction (Structural), Community Detection, Extraction Cache, God Nodes, Graphify Knowledge Graph Tool, Honesty Rules, Knowledge Graph Output, Semantic Extraction (LLM) (+28 more)

### Community 1 - "Compress Detection Utils"
Cohesion: 0.20
Nodes (17): Path, count_bullets(), extract_code_blocks(), extract_headings(), extract_inline_codes(), extract_paths(), extract_urls(), Line-based fenced code block extractor.      Handles ``` and ~~~ fences with var (+9 more)

### Community 2 - "Compress Detection OpenCode"
Cohesion: 0.20
Nodes (17): Path, count_bullets(), extract_code_blocks(), extract_headings(), extract_inline_codes(), extract_paths(), extract_urls(), Line-based fenced code block extractor.      Handles ``` and ~~~ fences with var (+9 more)

### Community 3 - "Caveman Agent Skills"
Cohesion: 0.08
Nodes (24): Caveman Compress Security, Boundaries, Caveman Compress, Compress, Compression Rules, Pattern, Preserve EXACTLY (never modify), Preserve Structure (+16 more)

### Community 4 - "Compress CLI Tool"
Cohesion: 0.18
Nodes (16): main(), print_usage(), Path, build_compress_prompt(), build_fix_prompt(), call_claude(), compress_file(), is_sensitive_path() (+8 more)

### Community 5 - "Filler DeepPro Plan"
Cohesion: 0.09
Nodes (26): Aggressive Heatmap (BFS from Opponent), Byte-Level Grid Storage Variant, Deterministic Tiebreak Rule (Lower Row then Lower Col), Negative-Offset Placement Search, Never Panic Guardrail, Strategy Tuning Guide, Test-Driven Development Cycle, 80% Win-Rate Threshold (+18 more)

### Community 6 - "File Type Detection Utils"
Cohesion: 0.24
Nodes (11): Path, detect_file_type(), _is_code_line(), _is_json_content(), _is_yaml_content(), Return True if the file is natural language and should be compressed., Check if a line looks like code., Check if content is valid JSON. (+3 more)

### Community 7 - "File Type Detection OpenCode"
Cohesion: 0.24
Nodes (11): Path, detect_file_type(), _is_code_line(), _is_json_content(), _is_yaml_content(), Return True if the file is natural language and should be compressed., Check if a line looks like code., Check if content is valid JSON. (+3 more)

### Community 8 - "Text Compression Logic"
Cohesion: 0.33
Nodes (9): Path, build_compress_prompt(), build_fix_prompt(), call_claude(), compress_file(), is_sensitive_path(), strip_llm_wrapper(), Heuristic denylist for files that must never be shipped to a third-party API. (+1 more)

### Community 9 - "Benchmark Utils"
Cohesion: 0.60
Nodes (5): Path, benchmark_pair(), count_tokens(), main(), print_table()

### Community 10 - "Benchmark OpenCode"
Cohesion: 0.60
Nodes (5): Path, benchmark_pair(), count_tokens(), main(), print_table()

### Community 11 - "Karpathy Guidelines"
Cohesion: 0.40
Nodes (5): Goal-Driven Execution, Simplicity First, Karpathy Guidelines Skill, Surgical Changes, Think Before Coding

### Community 12 - "No-Mistakes Pipeline"
Cohesion: 0.18
Nodes (12): AXI Command Family, Before you start, Escalate `ask-user` findings, Gate/Findings Decision System, Inspecting state, Intent is required, Intent Requirement, no-mistakes (+4 more)

### Community 17 - "Karpathy Rules"
Cohesion: 0.29
Nodes (6): Karpathy Guidelines, 1. Think Before Coding, 2. Simplicity First, 3. Surgical Changes, 4. Goal-Driven Execution, Karpathy Guidelines

### Community 24 - "Find Skills"
Cohesion: 0.14
Nodes (13): Common Skill Categories, Find Skills, How to Help Users Find Skills, Step 1: Understand What They Need, Step 2: Check the Leaderboard First, Step 3: Search for Skills, Step 4: Verify Quality Before Recommending, Step 5: Present Options to the User (+5 more)

### Community 29 - "Community 29"
Cohesion: 0.05
Nodes (41): 0. Project Structure, 10. Win-Rate Parser — `e2e/assert_winrate.rs`, 11. Dockerfile & Exclusions, 12. Build & Test Commands, 13. Full TDD Execution Order (Day-by-Day), 14. Edge Cases Checklist, 15. Audit Question → Test Mapping, 16. Visualizer Specification (Bonus) (+33 more)

### Community 30 - "Community 30"
Cohesion: 0.05
Nodes (37): Admin (requires admin access), Authentication, Branches, Checkout PR, Comments, Create Issue, Create PR, Create Release (+29 more)

### Community 31 - "Community 31"
Cohesion: 0.14
Nodes (23): Box, BufRead, mock_stdin(), Error, GameState, main(), Result, parse_anfield() (+15 more)

### Community 32 - "Community 32"
Cohesion: 0.08
Nodes (23): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+15 more)

### Community 33 - "Community 33"
Cohesion: 0.08
Nodes (23): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+15 more)

### Community 34 - "Community 34"
Cohesion: 0.09
Nodes (23): 10. `dev-dependencies` for Integration Tests, 11. Error Handling Rule: Never Panic on User Input, 12. `.dockerignore`, 13. Alpine-Based Docker (Smaller Image), 14. Split `strategy.rs` into `heatmap.rs` + `scorer.rs`, 15. Piece Block Character Consistency, 16. Byte-Level Grid Storage (Architectural Variant), 1. Negative-Offset Placement Search (+15 more)

### Community 35 - "Community 35"
Cohesion: 0.10
Nodes (20): 10. `dev-dependencies` for Integration Tests, 11. Error Handling Rule: Never Panic on User Input, 12. `.dockerignore`, 13. Alpine-Based Docker (Smaller Image), 14. Split `strategy.rs` into `heatmap.rs` + `scorer.rs`, 15. `Piece` Block Character Inconsistency, 1. Negative-Offset Search in `find_valid_placements` (Correctness), 2. Strategy Tuning Guide (Win-Rate Insurance) (+12 more)

### Community 36 - "Community 36"
Cohesion: 0.14
Nodes (13): Before / After, Benchmarks, How It Work, <img src="../../docs/assets/dancing-rock.svg" width="20" height="20" alt="rock"/> Caveman (285 tokens), Install, 📄 Original (706 tokens), Part of Caveman, Security (+5 more)

### Community 37 - "Community 37"
Cohesion: 0.14
Nodes (14): 3. Module B: Placement Validator — `src/validator.rs`, B1: Boundary Check, B2: Overlap Logic, B3: Find All Valid Placements, C1: Generate Heatmap (BFS from opponent territory), C2: Score a Placement, C3: Choose Best Placement, C4: Advanced Territory Control (Bonus) (+6 more)

### Community 38 - "Community 38"
Cohesion: 0.14
Nodes (14): 8.1. Benchmark Harness — `benches/turn_benchmark.rs`, 8.2. E2E Replay Validation Test — `tests/e2e.rs`, 8.3. Multi-Turn Territory Growth Test — `tests/multi_turn.rs`, 8.4. Deterministic Tie-Breaker Test — `tests/strategy_tests.rs`, 8. Integration Tests — `tests/integration_tests.rs`, Goal, IT-1: P1 single valid turn — places on own territory, IT-2: P2 single valid turn (+6 more)

### Community 40 - "Community 40"
Cohesion: 0.05
Nodes (37): +Are the tests checking each possible case?, Are there specific tests for **Input Parsing** (e.g., verifying the robot correctly reads the Anfield dimensions and the piece shape from stdin)?, Are there tests for **Boundary Detection** to ensure pieces are never placed partially outside the grid?, Are there tests for **Placement Validation** (e.g., checking that a move is rejected if it overlaps two of your own cells or one of the opponent's)?, Basic, Bonus, Can you confirm that the project runs correctly?, Can you confirm that the student player is placing the pieces correctly with the overlapping of just on cell? (+29 more)

### Community 41 - "Community 41"
Cohesion: 0.33
Nodes (12): build_test_grid(), find_valid_placements(), is_in_bounds(), is_valid_placement(), Grid, Piece, Player, Point (+4 more)

### Community 42 - "Community 42"
Cohesion: 0.26
Nodes (8): Self, Cell, GameState, Grid, Piece, Player, Point, Vec

### Community 43 - "Community 43"
Cohesion: 0.27
Nodes (11): choose_best_placement(), generate_heatmap(), Grid, Option, Piece, Player, Point, Vec (+3 more)

### Community 44 - "Community 44"
Cohesion: 0.05
Nodes (42): caveman-commit, Example output, How to invoke, See also, What it does, Caveman Commit Skill, Caveman Compress, caveman-help (+34 more)

### Community 46 - "Community 46"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 47 - "Community 47"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 49 - "Community 49"
Cohesion: 0.25
Nodes (7): Auth behavior, File size limit, Reporting a vulnerability, Security, Snyk High Risk Rating, What the skill does NOT do, What triggers the rating

### Community 50 - "Community 50"
Cohesion: 0.25
Nodes (7): 1. Test-Driven Development (TDD), 2. Robust Error Handling (Never Panic Guardrail), 3. Coordinate System & Offsets, 4. Input/Output (IO) Protocol, 5. Decision Performance Target, 6. Strategy & Deterministic Selection, DeepPro Methodology & Best Practices

### Community 51 - "Community 51"
Cohesion: 0.29
Nodes (6): Auto-clarity (inherited), Chaining patterns, Output contracts, What NOT to do, When to use cavecrew vs alternatives, Why this exists (the real win)

### Community 53 - "Community 53"
Cohesion: 0.62
Nodes (5): check_winrate(), green(), info(), red(), run_audit_suite.sh script

### Community 54 - "Community 54"
Cohesion: 0.38
Nodes (4): format_move(), format_no_move(), Point, String

### Community 56 - "Community 56"
Cohesion: 0.33
Nodes (5): 1. Think Before Coding, 2. Simplicity First, 3. Surgical Changes, 4. Goal-Driven Execution, Karpathy Guidelines

### Community 57 - "Community 57"
Cohesion: 0.24
Nodes (9): cavecrew, Example chaining, How to invoke, See also, What it does, Cavecrew Builder, Cavecrew Skill Definition, Cavecrew Investigator (+1 more)

### Community 61 - "Community 61"
Cohesion: 0.33
Nodes (5): 1. Think Before Coding, 2. Simplicity First, 3. Surgical Changes, 4. Goal-Driven Execution, Karpathy Guidelines

### Community 62 - "Community 62"
Cohesion: 0.40
Nodes (4): Auto-Clarity, Boundaries, Examples, Rules

### Community 63 - "Community 63"
Cohesion: 0.40
Nodes (4): Auto-Clarity, Boundaries, Examples, Rules

### Community 65 - "Community 65"
Cohesion: 0.40
Nodes (4): Meta Commands, RTK - Rust Token Killer (Google Antigravity), Rule, Why

### Community 66 - "Community 66"
Cohesion: 0.50
Nodes (3): Documentation & Implementation Tracking, graphify, karpathy-guidelines

### Community 67 - "Community 67"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 68 - "Community 68"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 69 - "Community 69"
Cohesion: 0.50
Nodes (3): For /graphify explain, For /graphify path, graphify reference: query, path, explain

### Community 70 - "Community 70"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 72 - "Community 72"
Cohesion: 0.50
Nodes (3): Meta commands (use directly), RTK — Token-Optimized CLI, Rule

### Community 73 - "Community 73"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 74 - "Community 74"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 75 - "Community 75"
Cohesion: 0.50
Nodes (3): For /graphify explain, For /graphify path, graphify reference: query, path, explain

### Community 76 - "Community 76"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 92 - "Community 92"
Cohesion: 0.21
Nodes (15): Duration, Frame, parse_anfield_header(), play(), read_replay(), render_frame(), Grid, R (+7 more)

## Knowledge Gaps
- **410 isolated node(s):** `$schema`, `plugin`, `@opencode-ai/plugin`, `Result`, `Box` (+405 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Filler Implementation Plan — TDD in Rust (DeepPro Edition)` connect `Community 29` to `Community 37`, `Filler DeepPro Plan`, `Community 38`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **Why does `Final Cross-Plan Analysis: Enrichments for Deeppro` connect `Community 34` to `Filler DeepPro Plan`?**
  _High betweenness centrality (0.011) - this node is a cross-community bridge._
- **Why does `Cross-Plan Analysis: Enrichments for Deeppro from Gem & GLM Plans` connect `Community 35` to `Filler DeepPro Plan`?**
  _High betweenness centrality (0.010) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Caveman Compress` (e.g. with `Compress` and `RTK CLI Proxy`) actually correct?**
  _`Caveman Compress` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Caveman compress scripts.  This package provides tools to compress natural langu`, `Split YAML frontmatter from body. Returns (frontmatter, body).      Memory files`, `Resolve the out-of-tree backup directory for a given source file.      Backups m` to the rest of the system?**
  _444 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Graphify Core Pipeline` be split into smaller, more focused modules?**
  _Cohesion score 0.06349206349206349 - nodes in this community are weakly interconnected._
- **Should `Caveman Agent Skills` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._