## karpathy-guidelines

Follow Karpathy's 4 principles: think before coding (state assumptions, surface tradeoffs), simplicity first (minimum code, no speculation), surgical changes (touch only what you must), goal-driven execution (define success criteria, loop until verified).

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, invoke the `skill` tool with `skill: "graphify"` before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

## Documentation & Implementation Tracking

Before performing implementation tasks, agents must reference and read the appropriate project documentation:
- **Implementation Tracker**: Always reference the checklist in [implementation_tracker.md](file:///home/ertval/code/zone-modules/filler/docs/plan/implementation_tracker.md) (also referred to as `@implementation-tracker`) to guide tasks and check off completed work.
- **DeepPro Design Plan**: Consult [deeppro.md](file:///home/ertval/code/zone-modules/filler/docs/plan/deeppro.md) for the core specifications, architecture, and step-by-step TDD guidelines.
- **Enrichments & Enhancements**: Refer to [final-cross-plan-enrichments.md](file:///home/ertval/code/zone-modules/filler/docs/report/final-cross-plan-enrichments.md) for critical bug fixes (such as negative coordinate support) and strategy enhancements.
