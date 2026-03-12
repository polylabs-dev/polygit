# Poly Git

PQ-signed version control with enterprise governance, built on eStream v0.9.1.

## Overview

Poly Git is an enterprise-grade git platform where every commit is PQ-signed (ML-DSA-87), every object is scatter-stored (k-of-n erasure-coded across providers/jurisdictions), and every push passes through lex-governed RBAC with AI-assisted code review. Built as a thin product layer over eStream's `es-git` CLI and three production FastLang governance circuits.

## Architecture

```
Developer Workflow (unchanged)
    |
    +-- git push poly main
    |
    v
poly-git Remote Helper
    |
    +-- ML-DSA-87 commit signing (replaces GPG)
    +-- .polyclassification enforcement
    |
    v
es-git CLI (estream/tools/es-git/)
    |
    +-- scatter-cas object storage
    +-- dual-write bridge (GitHub + scatter)
    |
    v
Enterprise Governance (FastLang)
    |
    +-- group_hierarchy.fl (org/group/repo containment)
    +-- rbac.fl (fine-grained permissions, role inheritance)
    +-- issue_tracking.fl (native issues, milestones, labels)
    |
    v
AI Code Review (Cortex)
    |
    +-- CodeReviewCircuit (risk scoring, dependency analysis)
    +-- ReviewApprovalCircuit (N-of-M signed approvals)
    +-- Blessed Repo Pattern (privacy-preserved contributor mapping)
```

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| poly-git remote helper | crates/poly-git/ | Git remote helper for scatter-distributed repos |
| Enterprise governance | circuits/fl/ | FastLang circuits for RBAC, org hierarchy, issues |
| Desktop App | apps/desktop/ | Tauri-based repo browser and admin console |
| CLI | crates/poly-git-cli/ | Command-line interface for poly-git operations |

## eStream Foundation

| Circuit / Spec | Location (estream) | What It Provides |
|----------------|-------------------|------------------|
| `group_hierarchy.fl` | `circuits/core/data/graphs/` | Org/group/repo containment, quota aggregation, visibility control |
| `rbac.fl` | `circuits/core/data/graphs/` | Role-based permissions with bitmask (read/write/admin/create_repo/delete_repo/manage_members/manage_ci), inheritance, expiry, audit |
| `issue_tracking.fl` | `circuits/core/data/graphs/` | Issue graph with state machine lifecycle, labels, milestones, blocking dependencies |
| `ESTREAM_GIT_SPEC.md` | `specs/core/data/` | AI code review, blessed repo pattern, governance-controlled builds, hybrid architecture |
| `es-git` CLI | `tools/es-git/` | Rust CLI: init, add, commit, log, status, branch, checkout, push, pull, clone, migrate, dual-write, verify |

## Classification

`.polyclassification` file controls scatter policy per path (like `.gitattributes`):

```
*.secret    classification=SOVEREIGN
docs/       classification=INTERNAL
*.md        classification=PUBLIC
```

## No REST API

All communication uses the eStream Wire Protocol. No REST/HTTP endpoints.

## Platform

- eStream v0.9.1
- FastLang circuits (graph/DAG constructs, state machines)
- ML-KEM-1024, ML-DSA-87, SHA3-256
- Stratum storage, Cortex AI governance
- 8-Dimension metering
- L2 multi-token payments

## Developer Language Story (v0.9.1)

eStream supports **7 languages** at full parity: Rust (native), Python (PyO3), TypeScript (WASM), Go (CGo), C++ (FFI), Swift (C bridging), and FastLang (native).

### Internal Development

- **FastLang first**: all new circuits and features are authored in FastLang (.fl) first
- **Six-language parity**: every FastLang feature must have equivalent API surface in Rust, Python, TypeScript, Go, C++, and Swift
- ESCIR operations power the compiler pipeline but are invisible to users
