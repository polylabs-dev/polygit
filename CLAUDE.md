# poly-git

**GitHub repo**: [polylabs-dev/poly-git](https://github.com/polylabs-dev/poly-git)
**Category**: FL-native SmartCircuit VCS product
**Platform**: eStream v0.11.0+
**Marketplace**: Published as `.escx` package

## Overview

poly-git is the eStream-native version control system, implemented 100% in FastLang as a SmartCircuit. It replaces the standalone `es-git` server (polyquantum/estream/tools/es-git/src/server.rs) with a marketplace-resolved circuit that runs inside `estream-node`.

The `es-git` CLI remains as the client, but its transport layer is rewritten to speak eStream Wire protocol (opcodes 0xA0-0xAC) with SPARK session authentication via `polykit-identity`.

## Architecture

- **100% FastLang** — all VCS logic is FL circuits
- **SmartCircuit** — resolved from marketplace by estream-node at startup
- **Wire protocol** — VCS operations carried on eStream Wire UDP (port 5000)
- **Stratum graph** — repo metadata, ACLs, branch policies stored in vcs_repo_graph
- **Cortex governance** — repo creation, ACL changes, branch protection are governance actions
- **FSM safety** — push/pull lifecycles use all 7 FSM safety features (WAL, rollback, timeout, retry, guard fail-closed, concurrent, bounded history)
- **PolyKit dependencies** — identity, metering, telemetry, rate-limiter, sanitize, eslm-classify, console

## Structure

```
poly-git/
├── circuits/
│   ├── data/          — VCS data types (objects.fl, manifest.fl, repo.fl)
│   ├── graphs/        — vcs_repo_graph (graph.fl) + RBAC (rbac.fl)
│   ├── lifecycle/     — Push/pull FSMs (router.fl) + trigger circuits (validators.fl)
│   ├── access/        — ACL enforcement (acl.fl) + rate limiting (rate_limit.fl)
│   ├── integration/   — PolyKit imports (metering.fl, telemetry.fl, sanitize.fl, classify.fl)
│   ├── signing/       — SPARK signing (manifest.fl)
│   ├── corpus/        — AI observation data (observation.fl)
│   ├── streams/       — Event streams (streams.fl)
│   ├── console/       — Admin widgets (widgets.fl)
│   └── governance/    — Governance actions (governance.fl)
├── estream-component.toml
└── CLAUDE.md
```

## Dependencies

- eStream platform v0.11.0+ (polyquantum/estream)
- polykit-identity@0.2.0 (marketplace)
- polykit-metering@0.2.0 (marketplace)
- polykit-telemetry@0.2.0 (marketplace)
- polykit-rate-limiter@0.2.0 (marketplace)
- polykit-sanitize@0.2.0 (marketplace)
- polykit-eslm-classify@0.2.0 (marketplace)
- polykit-console@0.2.0 (marketplace)

## Workflow

This is a standalone git repository. Commit work to the GitHub issue or epic it was done under.

## Cross-Repo Coordination

- `toddrooke/ai-toolkit/CLAUDE-CONTEXT.md` — org map and priorities
- `toddrooke/ai-toolkit/scratch/BACKLOG.md` — master backlog
