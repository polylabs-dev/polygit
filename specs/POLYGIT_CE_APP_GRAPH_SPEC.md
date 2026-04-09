# PolyGit Cognitive Engine + App Graph Specification

| Field | Value |
|-------|-------|
| **Version** | v0.1.0 |
| **Status** | Draft |
| **Lex Namespace** | `polyqlabs/qgit` |
| **App Graph** | `circuits/fl/qgit_app_graph.fl` |
| **Meaning Domains** | `circuits/fl/qgit_meaning.fl` |
| **Upstream Dependency** | eStream v0.22.0+, QKit v0.12.0+ |

---

## 1. Overview

PolyGit is a PQ-signed version control product built on eStream es-vcs, consisting of 32 circuits across two layers: 12 product FL circuits, 1 graph (repo_graph), and 21 SmartCircuit layer circuits. This spec defines the Cognitive Engine (CE) integration and App Graph registration for PolyGit, enabling meaning-aware code review quality analysis, CI pipeline optimization, and governance compliance monitoring.

The App Graph registers 13 product-level modules into the eStream module stratum, declares intra-graph dependency edges (EDGE_REQUIRES), and establishes cross-graph bridge edges to es-vcs, QKit CI, and PolyDocs. The CE integration adds three meaning domains that crystallize patterns from commit history, code review activity, and governance compliance into actionable insights surfaced via SME panels.

---

## 2. App Graph Module Inventory

13 modules registered under `polyqlabs/qgit`:

| Module | Aperture Partition | SLA Tier | Entitlement Tier |
|--------|-------------------|----------|------------------|
| `qgit_branch` | Backend | Premium | Pro |
| `qgit_ci` | Backend | Premium | Pro |
| `qgit_commit` | Backend | Premium | Pro |
| `qgit_document_review` | Backend | Standard | Pro |
| `qgit_governance` | Backend | Premium | Enterprise |
| `qgit_issue` | Backend | Standard | Pro |
| `qgit_lfs` | Backend | Standard | Pro |
| `qgit_metering` | Shared | Standard | Pro |
| `qgit_platform_health` | Shared | Premium | Pro |
| `qgit_rbac` | Backend | Premium | Enterprise |
| `qgit_repository` | Backend | Premium | Pro |
| `qgit_review` | Backend | Premium | Pro |
| `qgit_repo_graph` | Backend | Premium | Enterprise |

Each module carries a `ResourceBudget8D` with execution, hash_ops, bandwidth, storage, observe_events, proofs, consensus, and memory allocations. Budget enforcement is Hard for all modules.

---

## 3. CE Meaning Domains

Three meaning domains under `polyqlabs/qgit/cognitive`:

| Domain | Crystallization Threshold | Impact Weight | Description |
|--------|--------------------------|---------------|-------------|
| `vcs/commit_patterns` | 50 events | 0.3 | Commit frequency distributions, branch lifecycle analysis (creation-to-merge latency, stale branch detection), merge conflict hotspot identification, committer cadence anomalies |
| `vcs/code_quality` | 30 events | 0.4 | Code review velocity (time-to-first-review, time-to-approval), CI pass rate trends, test coverage delta per PR, review depth scoring (comments-per-diff-line), automated vs human review ratio |
| `vcs/governance` | 20 events | 0.3 | Approval pattern compliance (required approvers met vs bypassed), policy exception frequency, branch protection rule adherence, CODEOWNERS coverage gaps, governance escalation patterns |

Crystallization thresholds define the minimum event count before the CE begins pattern extraction. Impact weights determine relative contribution to the product-level insight score (weights sum to 1.0).

---

## 4. Noise Filter Configuration

The noise filter separates actionable signal from high-volume automated activity:

**Suppressed (noise):**

- Bot commits (dependency bumps, auto-formatters, CI bot commits) — identified by committer identity matching bot registry
- CI pipeline noise (retry storms, flaky test retries, cache invalidation rebuilds) — filtered by pipeline run metadata
- Auto-generated code (protobuf stubs, schema migrations, lock files) — filtered by file path patterns and commit message prefixes

**Amplified (signal):**

- Review disagreements — reviewer approvals overridden or conflicting review verdicts on the same PR
- Governance exceptions — branch protection bypasses, force-pushes to protected branches, approval requirement overrides
- Security alerts — commits introducing known vulnerable dependencies, secrets detected in diffs, unsigned commits to signed-required branches

---

## 5. SME Panels

Two SME panels provide domain-expert-level insight dashboards:

### 5.1 Code Review Quality

- **Domain sources**: `vcs/code_quality`, `vcs/commit_patterns`
- **Metrics surfaced**: Review throughput (PRs reviewed/day), review depth score, time-to-first-review p50/p95, approval rubber-stamping detection, reviewer load balancing index
- **Triggers**: Review depth drops below threshold for 5 consecutive PRs, single reviewer approving >60% of team PRs

### 5.2 CI Pipeline Optimization

- **Domain sources**: `vcs/code_quality`, `vcs/commit_patterns`
- **Metrics surfaced**: Pipeline duration p50/p95/p99, flaky test identification, cache hit rate, parallelization efficiency, failure root cause clustering
- **Triggers**: Pipeline duration p95 exceeds SLA threshold, flaky test rate exceeds 5%, cache hit rate drops below 70%

---

## 6. Bridge Edges

Cross-graph bridges connect PolyGit modules to external eStream and PolyQ Labs graphs:

| Source Module | Target Graph | Target Module | Bridge Type |
|---------------|-------------|---------------|-------------|
| `qgit_commit` | eStream es-vcs | `es_vcs_core` | `EDGE_BRIDGE_TO` — commit wire protocol delegation |
| `qgit_ci` | QKit CI | `qkit_ci_runner` | `EDGE_BRIDGE_TO` — pipeline execution dispatch |
| `qgit_document_review` | PolyDocs | `qdocs_review` | `EDGE_BRIDGE_TO` — document review integration |

Bridge edges are registered via `qgit_register_bridge_edges()` and carry `BridgeEdgeData` metadata encoding source/target lex namespaces, module names, and bridge type semantics.

---

## 7. Strategic Grant Configuration

PolyGit participates in two strategic grant relationships:

### 7.1 eStream Platform Grant

- **Grantor**: eStream (lex `core/platform`)
- **Grant type**: Platform — PolyGit is a first-party PolyQ Labs product built on eStream primitives
- **Scope**: Full access to es-vcs wire protocol, scatter-cas, SPARK identity, StreamSight observability
- **Governance**: GOVERNANCE_OBSERVE edges from eStream governance to all 13 PolyGit modules

### 7.2 Paragon Strategic Partner Grant

- **Grantor**: Paragon (lex `paragon/*`)
- **Grant type**: Strategic Partner — PolyGit provides version control for Paragon's legal document workflows
- **Scope**: Bridge edges for document review integration, governance audit trail
- **Governance**: Read-only observation, no write access to Paragon lex namespaces

---

## 8. Edge Summary

| Edge Type | Count | Description |
|-----------|-------|-------------|
| `EDGE_REQUIRES` | ~18 | Intra-graph module dependencies |
| `EDGE_BRIDGE_TO` | 3 | Cross-graph bridges (es-vcs, QKit CI, PolyDocs) |
| `EDGE_GOVERNANCE_OBSERVE` | 13 | Governance observation of all modules |
| `EDGE_COMPILED_BY` | 13 | Compilation provenance for all modules |
| **Total** | ~47 | |

---

## References

- `circuits/fl/qgit_app_graph.fl` — App graph registration circuit
- `circuits/fl/qgit_meaning.fl` — CE meaning domain circuit
- `estream-component.toml` — Marketplace manifest
- eStream `core/deployment/module_stratum.fl` — Module stratum types
- eStream `core/deployment/cross_graph_edges.fl` — Cross-graph edge types
