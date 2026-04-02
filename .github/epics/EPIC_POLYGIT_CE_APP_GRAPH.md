# Epic: PolyGit Cognitive Engine + App Graph Integration

| Field | Value |
|-------|-------|
| **Status** | Planned |
| **Priority** | P0 |
| **Product** | PolyGit |
| **Depends On** | PolyKit CE Framework, eStream CE Implementation |
| **Spec** | `specs/POLYGIT_CE_APP_GRAPH_SPEC.md` |

---

## Overview

Integrate PolyGit with the eStream Cognitive Engine (CE) and App Graph module stratum. This registers PolyGit's 13 product-level modules into the platform graph, establishes cross-graph bridge edges to es-vcs, PolyKit CI, and PolyDocs, and activates 3 CE meaning domains for commit pattern analysis, code quality tracking, and governance compliance monitoring. Two SME panels surface actionable insights for code review quality and CI pipeline optimization.

---

## Tasks

- [ ] Create `specs/POLYGIT_CE_APP_GRAPH_SPEC.md` — CE + App Graph integration spec
- [ ] Create `circuits/fl/polygit_app_graph.fl` — 13 ModuleNode declarations, EDGE_REQUIRES dependency edges, cross-graph bridge registration, governance edge registration
- [ ] Create `circuits/fl/polygit_meaning.fl` — 3 meaning domains (`vcs/commit_patterns`, `vcs/code_quality`, `vcs/governance`), noise filter, 2 SME panels, orchestrator circuit
- [ ] Update `estream-component.toml` with strategic grant declarations for eStream platform grant and Paragon partner grant
- [ ] Golden tests for `polygit_app_graph.fl` — module count, edge count, bridge edges, governance edges, module lookup
- [ ] Golden tests for `polygit_meaning.fl` — domain validity (weights sum to 1.0), noise filter rules, SME panel configs, orchestrator registration
- [ ] Update `CLAUDE.md` with CE conventions — meaning domain naming, noise filter patterns, SME panel configuration guidelines

---

## Exit Criteria

- All 13 modules registered in app graph with correct aperture partitions, SLA tiers, and resource budgets
- 18 EDGE_REQUIRES intra-graph dependency edges established
- 3 EDGE_BRIDGE_TO cross-graph edges (es-vcs, PolyKit CI, PolyDocs) registered
- 13 GOVERNANCE_OBSERVE edges from platform governance to all PolyGit modules
- 3 CE meaning domains active with crystallization thresholds (50/30/20 events)
- Impact weights sum to 1.0 across all meaning domains
- Noise filter suppresses bot commits, CI noise, auto-generated code
- 2 SME panels configured with triggers for review quality degradation and CI SLA breaches
- All golden tests pass

---

## Dependencies

### PolyKit Circuits

- `polylabs/polykit/cognitive.fl` — CE framework: `register_meaning_domain`, `register_noise_filter`, `register_sme_panel`
- `polylabs/polykit/ci.fl` — CI runner bridge target (`polykit_ci_runner`)

### eStream Core

- `core/deployment/module_stratum.fl` — `ModuleNode`, `ModuleEdge`, `CsrStorage`, edge type constants
- `core/deployment/cross_graph_edges.fl` — `GovernanceObserveEdge`, `CompilationProvenanceEdge`
- `core/vcs/` — es-vcs bridge target (`es_vcs_core`)
