# Poly Git Architecture

**Version**: 1.0
**Last Updated**: February 2026

---

## Overview

Poly Git is PQ-signed version control with enterprise governance. It wraps eStream's `es-git` CLI to provide a git-compatible workflow where every object is scatter-stored, every commit is ML-DSA-87 signed, and every operation passes through lex-governed RBAC.

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Poly Git Architecture                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  DEVELOPER WORKFLOW (unchanged)                                         │
│  ──────────────────────────────                                         │
│  git remote add poly poly-git://org/repo                               │
│  git push poly main                                                     │
│  git pull poly main                                                     │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────┐           │
│  │              poly-git Remote Helper                      │           │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │           │
│  │  │ PQ Signing    │  │ Classification│  │ Object       │  │           │
│  │  │ ML-DSA-87     │  │ Enforcement   │  │ Chunking     │  │           │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │           │
│  └──────────────────────┬──────────────────────────────────┘           │
│                          │                                              │
│  ┌───────────────────────v──────────────────────────────────┐          │
│  │                   es-git CLI (Rust)                        │          │
│  │  init │ add │ commit │ log │ status │ branch │ checkout   │          │
│  │  push │ pull │ clone │ migrate │ dual-write │ verify      │          │
│  │                                                            │          │
│  │  Backed by: estream-scatter-cas                            │          │
│  └───────────────────────┬────────────────────────────────────┘          │
│                          │                                              │
│  ┌───────────────────────v──────────────────────────────────┐          │
│  │              Enterprise Governance (FastLang)              │          │
│  │                                                            │          │
│  │  ┌──────────────────┐  ┌──────────────────┐              │          │
│  │  │ group_hierarchy  │  │      rbac        │              │          │
│  │  │                  │  │                  │              │          │
│  │  │ graph {          │  │ graph {          │              │          │
│  │  │   OrgNode        │  │   RoleNode       │              │          │
│  │  │   GroupNode      │  │   PermissionSet  │              │          │
│  │  │   RepoNode       │  │   RoleAssignment │              │          │
│  │  │   ContainmentEdge│  │   InheritanceEdge│              │          │
│  │  │ }                │  │ }                │              │          │
│  │  └──────────────────┘  └──────────────────┘              │          │
│  │                                                            │          │
│  │  ┌──────────────────┐  ┌──────────────────┐              │          │
│  │  │ issue_tracking   │  │  AI Code Review  │              │          │
│  │  │                  │  │                  │              │          │
│  │  │ state_machine {  │  │ CodeReviewCircuit│              │          │
│  │  │   OPEN →         │  │ ReviewApproval   │              │          │
│  │  │   IN_PROGRESS →  │  │ CommitPromotion  │              │          │
│  │  │   REVIEW →       │  │ BlessedRepo      │              │          │
│  │  │   CLOSED →       │  │                  │              │          │
│  │  │   ARCHIVED       │  │                  │              │          │
│  │  │ }                │  │                  │              │          │
│  │  └──────────────────┘  └──────────────────┘              │          │
│  └────────────────────────────────────────────────────────────┘          │
│                                                                         │
│  ┌──────────────────────────────────────────────────────────┐          │
│  │                    Scatter Storage                         │          │
│  │  k-of-n erasure coding across providers/jurisdictions     │          │
│  │                                                            │          │
│  │  AWS (US) │ GCP (EU) │ Azure (APAC) │ Hetzner │ CF       │          │
│  └──────────────────────────────────────────────────────────┘          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. poly-git Remote Helper

A git remote helper that intercepts standard git operations and routes them through the eStream stack.

**Responsibilities**:
- ML-DSA-87 commit signing (replaces GPG)
- `.polyclassification` enforcement per path
- Object chunking and scatter-cas storage
- Dual-write bridge: push to both GitHub and scatter-cas simultaneously

**Crate**: `crates/poly-git/`

### 2. es-git CLI (Platform Layer)

The existing eStream CLI tool at `estream/tools/es-git/` provides the storage backend. Poly Git wraps this with product-level UX and governance.

**Commands**: init, add, commit, log, status, branch, checkout, push, pull, clone, migrate, dual-write, verify

**Storage**: All git objects (blobs, trees, commits, tags) are stored as scatter-cas content-addressed chunks.

### 3. Enterprise Governance Circuits (FastLang)

Three production FastLang circuits from eStream provide the governance backbone:

#### group_hierarchy.fl

```
graph group_hierarchy {
    node OrgNode        -- top-level organization
    node GroupNode       -- nested groups (GitLab-style)
    node RepoNode        -- individual repositories
    edge ContainmentEdge -- parent-child containment

    overlay layers:
      quota_usage        -- storage/compute quota tracking
      member_count       -- membership aggregation
      visibility         -- public/internal/private
      inheritance_mask   -- permission inheritance control
}
```

**Circuits**: `create_org`, `create_group`, `add_repo`, `move_group`, `move_repo`, `aggregate_quota`, `list_repos`, `delete_group`

#### rbac.fl

```
graph rbac {
    node RoleNode        -- named roles with PermissionSet
    edge InheritanceEdge -- role inheritance chain

    PermissionSet bitmask:
      read | write | admin | create_repo | delete_repo | manage_members | manage_ci

    overlay layers:
      permissions         -- per-node permission resolution
      scope_binding       -- role-to-scope mapping
      principal_map       -- principal-to-role mapping
      role_inheritance    -- transitive inheritance
      audit_trail         -- complete audit log
      expiry              -- time-bounded assignments
}
```

**Circuits**: `assign_role`, `revoke_role`, `check_permission`, `resolve_permissions`, `propagate_permissions`, `audit_principal`, `expire_stale`

#### issue_tracking.fl

```
graph issue_graph {
    node IssueNode       -- issues with lifecycle state
    node LabelNode       -- classification labels
    node MilestoneNode   -- release milestones
    edge IssueRelationEdge -- blocking/relates-to/duplicates

    state_machine issue_lifecycle:
      OPEN -> IN_PROGRESS -> REVIEW -> CLOSED -> ARCHIVED
      anomaly detection on state transitions
}
```

**Circuits**: `create_issue`, `update_issue`, `close_issue`, `link_issues`, `assign_milestone`, `update_labels`, `issues_by_milestone`, `blocked_ordering` (topological sort), `vote_issue`

### 4. AI Code Review (Cortex)

From `ESTREAM_GIT_SPEC.md`:

- **CodeReviewCircuit**: AI-powered risk scoring, dependency analysis, test coverage verification
- **ReviewApprovalCircuit**: N-of-M ML-DSA-87 signed approvals required before merge
- **CommitPromotionCircuit**: Privacy-preserved contributor mapping (public/private attribution split)
- **Blessed Repo Pattern**: Governance-controlled push access to production repositories

Configuration via `.estream/review-config.yaml` and `.estream/promotion-config.yaml`.

---

## Hybrid Architecture

Poly Git supports a dual-mode deployment:

### Mode 1: GitHub + Scatter Overlay

Developers use GitHub as their primary workflow. The poly-git bridge:
1. Intercepts GitHub webhooks
2. PQ-signs all commits
3. Mirrors all objects to scatter-cas
4. Runs governance circuits on push events

### Mode 2: Pure Scatter (Self-Hosted)

Enterprise runs their own eStream node. All git operations go directly through es-git to scatter-cas. No GitHub dependency.

---

## Data Model

```
Repository (scatter-cas root)
├── Objects (blobs, trees, commits, tags)
│   └── Each object → scatter-cas chunk (k-of-n)
├── Refs (branches, tags, HEAD)
│   └── Stored on Stratum KV layer
├── Governance State
│   ├── RBAC graph (rbac.fl state)
│   ├── Org hierarchy (group_hierarchy.fl state)
│   └── Issue graph (issue_tracking.fl state)
└── Classification Policy
    └── .polyclassification → per-path scatter policy
```

---

## Phases

### Phase 1: Core Git Operations
- poly-git remote helper wrapping es-git
- ML-DSA-87 commit signing
- .polyclassification enforcement
- Basic scatter storage

### Phase 2: Collaboration
- Issue tracking (issue_tracking.fl)
- Pull request workflow
- Merge conflict resolution
- Code review (basic)

### Phase 3: Enterprise Governance
- Org/group hierarchy (group_hierarchy.fl)
- Fine-grained RBAC (rbac.fl)
- AI code review (Cortex)
- Blessed repo pattern
- Audit trail and compliance

### Phase 4: Console & Marketplace
- Admin console widget
- Marketplace .escx packaging
- Self-hosted enterprise deployment
- GitHub migration tooling

---

## Stratum & Cortex Integration

Poly Git's graph constructs compose Stratum storage bindings and Cortex AI governance at the data-declaration level. Every node type in the repo registry declares its storage tier, lex governance path, and Cortex visibility policy inline — no separate configuration layer.

### Stratum Storage Bindings

All graph and series data flows through Stratum's tiered CSR (Compressed Sparse Row) engine:

| Tier | Backing | Purpose |
|------|---------|---------|
| `hot @bram` | Block RAM (FPGA) / L1 cache | Active repo metadata, recent overlays, live contributor edges |
| `warm @ddr` | DDR5 DRAM | Full repo registry, org hierarchy, contributor stats |
| `cold @nvme` | NVMe SSD / scatter-cas | Archived repos, historical series, audit trail |

**Series**: `repo_series` captures every graph mutation with `merkle_chain true` (hash-chained), `lattice_imprint true` (lattice-timestamped), and `witness_attest true` (PoVC-witnessed). This provides a tamper-evident audit log for all repo registry changes.

**Overlay Curation**: All overlays use `delta_curate` for incremental propagation — commit counts, size bytes, and contributor counts stream to warm/cold tiers only when deltas exceed the curation threshold. Classification overlays use `curate delta_curate` for both snapshot and delta propagation.

### Cortex Visibility Policies

Each `data` declaration carries a `cortex {}` block that governs what Cortex (the AI inference layer) can see:

| Data Type | Policy | Effect |
|-----------|--------|--------|
| **RepoNode** | `obfuscate [owner_id]`, `infer on_write`, `on_anomaly alert "git-team"` | Cortex sees repo metadata for anomaly detection but owner identity is hashed. Push anomalies alert the git ops team. |
| **OrgNode** | `obfuscate [admin_id]`, `infer on_write` | Admin identity is hashed for org-level inference. No anomaly handler (org changes are low-frequency). |
| **GroupNode** | `infer on_write` | Full visibility for group hierarchy inference. No sensitive fields. |
| **ContributorNode** | `redact [email]`, `obfuscate [user_id]`, `infer on_write` | Email is fully stripped from Cortex context. User ID is hashed. Enables contributor churn detection without PII exposure. |

### Inference Triggers

- **`infer on_write`**: All node types trigger Cortex inference on mutation. The `repo_anomaly` AI feed scores push frequency, size growth rate, contributor churn, and classification changes against the `cortex_eslm` model.
- **Anomaly threshold**: 0.8 across the `repo_registry` observation window (3600s baseline). Anomalies detected by the `detect_push_anomalies` circuit feed back into the `repo_anomaly` AI feed.

### Feedback Handlers

- **`on_anomaly alert "git-team"`**: RepoNode anomalies (unusual push patterns, rapid size growth, classification changes) route to the git-team alert channel via StreamSight.
- **State machine integration**: `repo_lifecycle` transitions feed into anomaly detection (`li_anomaly_detection true`). Rapid CREATED→DELETED cycles or mass archival trigger Cortex scoring.

### Quantum State (.q) Capability

All graph data is `.q`-ready: the `series repo_series` with `merkle_chain true` + `lattice_imprint true` provides the cryptographic substrate for quantum-state snapshots. When hardware targets support it, the Stratum CSR engine can checkpoint graph state into `.q` quantum-committed snapshots — each snapshot is a lattice-imprinted, witness-attested point-in-time that can be verified against the merkle chain without replaying the full mutation history.

---

## Platform Dependencies

| Dependency | Source | Purpose |
|-----------|--------|---------|
| es-git CLI | `estream/tools/es-git/` | Core git operations on scatter-cas |
| scatter-cas | `estream/crates/estream-scatter-cas/` | Content-addressable storage |
| group_hierarchy.fl | `estream/circuits/core/data/graphs/` | Org containment |
| rbac.fl | `estream/circuits/core/data/graphs/` | Access control |
| issue_tracking.fl | `estream/circuits/core/data/graphs/` | Issue management |
| ESTREAM_GIT_SPEC.md | `estream/specs/core/data/` | AI review, blessed repo, governance |
| Stratum | eStream storage layer | KV state for refs and governance |
| Cortex | eStream AI layer | Code review, anomaly detection |
| StreamSight | eStream observability | Metrics, anomaly detection |
