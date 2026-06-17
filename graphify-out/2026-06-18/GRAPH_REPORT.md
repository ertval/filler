# Graph Report - .  (2026-06-18)

## Corpus Check
- Corpus is ~46,198 words - fits in a single context window. You may not need a graph.

## Summary
- 217 nodes · 300 edges · 29 communities (16 shown, 13 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 17 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

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

## God Nodes (most connected - your core abstractions)
1. `Graphify Knowledge Graph Tool` - 30 edges
2. `Filler Deeppro Implementation Plan` - 13 edges
3. `validate()` - 10 edges
4. `validate()` - 10 edges
5. `compress_file()` - 8 edges
6. `detect_file_type()` - 7 edges
7. `detect_file_type()` - 7 edges
8. `Final Cross-Plan Enrichments Report` - 7 edges
9. `backup_dir_for()` - 6 edges
10. `compress_file()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `Karpathy Guidelines` --semantically_similar_to--> `Karpathy Guidelines Rules`  [INFERRED] [semantically similar]
  AGENTS.md → .agents/rules/karpathy-guidelines.md
- `Graphify Knowledge Graph` --semantically_similar_to--> `Graphify Rules`  [INFERRED] [semantically similar]
  AGENTS.md → .agents/rules/graphify.md
- `Caveman Communication Mode` --semantically_similar_to--> `Caveman Communication Mode`  [INFERRED] [semantically similar]
  .github/copilot-instructions.md → .opencode/AGENTS.md
- `Compress Skill` --semantically_similar_to--> `Caveman Compress Skill`  [INFERRED] [semantically similar]
  .agents/skills/compress/SKILL.md → .agents/skills/caveman-compress/SKILL.md
- `Karpathy Guidelines` --conceptually_related_to--> `Graphify Knowledge Graph Tool`  [INFERRED]
  .agents/skills/karpathy-guidelines/SKILL.md → .opencode/skills/graphify/SKILL.md

## Import Cycles
- None detected.

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

## Communities (29 total, 13 thin omitted)

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
Cohesion: 0.14
Nodes (21): Cavecrew, Cavecrew Builder, Cavecrew Skill Definition, Cavecrew Investigator, Cavecrew Reviewer, Caveman Commit, Caveman Commit Skill, Caveman Compress (+13 more)

### Community 4 - "Compress CLI Tool"
Cohesion: 0.18
Nodes (16): main(), print_usage(), Path, build_compress_prompt(), build_fix_prompt(), call_claude(), compress_file(), is_sensitive_path() (+8 more)

### Community 5 - "Filler DeepPro Plan"
Cohesion: 0.24
Nodes (16): Filler Deeppro Implementation Plan, Aggressive Heatmap (BFS from Opponent), Byte-Level Grid Storage Variant, Deterministic Tiebreak Rule (Lower Row then Lower Col), Negative-Offset Placement Search, Never Panic Guardrail, Strategy Tuning Guide, Test-Driven Development Cycle (+8 more)

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
Cohesion: 0.60
Nodes (5): AXI Command Family, Gate/Findings Decision System, Intent Requirement, No-Mistakes Validation Pipeline, TOON Output Format

## Knowledge Gaps
- **40 isolated node(s):** `$schema`, `plugin`, `@opencode-ai/plugin`, `Graphify Knowledge Graph`, `RTK CLI Proxy` (+35 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **13 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Are the 8 inferred relationships involving `Filler Deeppro Implementation Plan` (e.g. with `Aggressive Heatmap (BFS from Opponent)` and `Byte-Level Grid Storage Variant`) actually correct?**
  _`Filler Deeppro Implementation Plan` has 8 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Caveman compress scripts.  This package provides tools to compress natural langu`, `Split YAML frontmatter from body. Returns (frontmatter, body).      Memory files`, `Resolve the out-of-tree backup directory for a given source file.      Backups m` to the rest of the system?**
  _74 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Graphify Core Pipeline` be split into smaller, more focused modules?**
  _Cohesion score 0.06349206349206349 - nodes in this community are weakly interconnected._
- **Should `Caveman Agent Skills` be split into smaller, more focused modules?**
  _Cohesion score 0.1380952380952381 - nodes in this community are weakly interconnected._