# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

## Operational Parameters
* **System Baseline Release Tag:** `v1.3.0-exec`
* **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Status:** FROZEN (ACTIVE)

## Architecture Overview
This open-core repository houses the bare-metal validation primitives and Subsystem 9 specifications required to drive deterministic, fail-closed runtime security environments.

```
src/
├── lib.rs          -> Hardware envelope declarations (S_max <= 4096B)
├── lifecycle.rs    -> Temporal ceiling pre-filtering (tau_override <= 11.99ms)
└── lattice.rs      -> 5-Tier Poset Dominance Lattice Enforcement
```

## Invariant Posture
All operations comply with the zero operational drag mandate ($C_{\text{ops}} = 0$). Academic staging matrices, private analytical sets, and vaulted Path C defense implementations are strictly excluded from this open distribution layer.
