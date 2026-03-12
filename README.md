# poly-git

Post-quantum version control as an eStream SmartCircuit. Replaces the standalone `es-git` server with a marketplace-resolved FastLang circuit running natively inside `estream-node`.

## Architecture

```
es-git CLI ──(estream:// Wire protocol)──> estream-node
                                              │
                                    ┌─────────┴──────────┐
                                    │ poly-git SmartCircuit│
                                    │   (marketplace pkg)  │
                                    └─────────┬──────────┘
                                              │
                          ┌───────────────────┼───────────────────┐
                          │                   │                   │
                     Stratum KV          Stratum Graph       Cortex Rules
                    (objects, refs)    (repo config, ACL,    (governance,
                                        branch policies)     anomaly detection)
```

## Features

- **100% FastLang-native** — all VCS logic is expressed in FL v0.11.0
- **SPARK authentication** — ML-DSA-87 identity derived from SPARK master seed
- **7 FSM safety features** — WAL, rollback, timeout, retry, guard fail-closed, concurrent FSM, bounded history
- **Graph-based repo config** — Stratum Graph stores repos, branches, ACLs, policies
- **Governance actions** — repo creation as genesis event, all config changes are Cortex-governed
- **AI-native** — `ai_feed` directives, corpus observations, anomaly detection on FSMs
- **Wire protocol** — VCS opcodes 0xA0-0xAC on the standard eStream UDP wire (port 5000)
- **PolyKit integration** — identity, metering, telemetry, rate limiting, sanitization, classification, console

## Circuit Layout

```
circuits/
├── data/           — VCS object types, repo config, push/pull manifests
│   ├── objects.fl
│   ├── repo.fl
│   └── manifest.fl
├── graphs/         — Stratum Graph definitions (repo graph, RBAC, governance)
│   ├── graph.fl
│   ├── rbac.fl
│   └── governance.fl
├── lifecycle/      — Push/pull FSMs with all safety features + guard circuits
│   ├── router.fl
│   └── validators.fl
├── access/         — ACL enforcement + rate limiting
│   ├── acl.fl
│   └── rate_limit.fl
├── integration/    — PolyKit service bindings (metering, telemetry, sanitize, classify)
│   ├── metering.fl
│   ├── telemetry.fl
│   ├── sanitize.fl
│   └── classify.fl
├── signing/        — SPARK signing via polykit-identity
│   └── manifest.fl
├── corpus/         — AI observation data for accretive intelligence
│   └── observation.fl
├── streams/        — Event stream definitions with @streamsight
│   └── streams.fl
├── console/        — Admin widgets (push activity, health, ACL, governance audit)
│   └── widgets.fl
└── tests/          — Journey tests (push, pull, rollback, timeout, governance)
    └── journeys.fl
```

## Dependencies (Marketplace)

| Package | Version | Purpose |
|---------|---------|---------|
| polykit-identity | ^0.2.0 | SPARK auth, ML-DSA-87 signing |
| polykit-metering | ^0.2.0 | 8-dimension resource metering |
| polykit-telemetry | ^0.2.0 | StreamSight telemetry pipeline |
| polykit-rate-limiter | ^0.2.0 | FIFO rate limiter with backpressure |
| polykit-sanitize | ^0.2.0 | 3-stage compliance sanitization |
| polykit-eslm-classify | ^0.2.0 | ESLM content classification |
| polykit-console | ^0.2.0 | Console widget framework + RBAC |

## Wire Protocol

The `es-git` CLI communicates with poly-git via eStream Wire protocol:

| Opcode | Direction | Payload | Purpose |
|--------|-----------|---------|---------|
| 0xA0 | C→S | VcsConnect | Session handshake with repo path |
| 0xA1 | S→C | VcsConnectAck | Session confirmation |
| 0xA2 | C→S | VcsRefList | Request ref listing |
| 0xA3 | S→C | VcsRefListResp | Ref listing response |
| 0xA4 | C→S | VcsHaveCheck | Object existence query |
| 0xA5 | S→C | VcsHaveResp | Have/want sets |
| 0xA6 | C→S | VcsSendObject | Push object (SPARK-signed) |
| 0xA7 | S→C | VcsSendAck | Object accepted/rejected |
| 0xA8 | C→S | VcsRequestObjects | Pull request |
| 0xA9 | S→C | VcsObjectsResp | Objects payload |
| 0xAA | C→S | VcsRefUpdate | Ref pointer update (signed) |
| 0xAB | S→C | VcsRefUpdateAck | Update result |
| 0xAC | S→C | VcsError | Error response |

## Usage

```bash
# Clone via eStream Wire protocol (poly-git on node)
es-git clone estream://node1.alpha-devnet:5000/polylabs-dev/my-repo

# Push (SPARK-authenticated, all objects signed)
es-git push --remote estream://node1.alpha-devnet:5000/polylabs-dev/my-repo

# Pull
es-git pull --remote estream://node1.alpha-devnet:5000/polylabs-dev/my-repo

# Verify (includes marketplace attestation check)
es-git verify
```

## Marketplace

Published as `poly-git@0.1.0` on the eStream marketplace. The `estream-node` resolves and loads this circuit automatically when VCS Wire packets arrive.

```toml
# estream-component.toml
[component]
name = "poly-git"
version = "0.1.0"
category = "smart-circuit"
implementation_type = "FastLang"
```

## Development

This repo is pure FastLang. There is no Rust code — all Rust integration is in `polyquantum/estream` (Wire protocol, node dispatch, `es-git` CLI).

To run journey tests (when `estream-dev test --journey` is available):

```bash
estream-dev test --journey poly-git
```

## License

Proprietary — PolyQuantum Foundation. Licensed to Poly Labs via Commercial Services Agreement.
