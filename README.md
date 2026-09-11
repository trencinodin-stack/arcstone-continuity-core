Markdown
# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

## Operational Parameters
* **System Baseline Release Tag:** `v1.3.1-exec`
* **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Crate Version:** `1.3.1` (Zero Heap Dependencies)
* **Status:** FROZEN (ACTIVE)

## Architectural Scope & Public Boundary
This repository represents the open-core reference baseline for Subsystem 9 execution membranes. To maintain zero operational drag (**C_ops = 0**) and absolute memory bounds (**S_max <= 4096B**), this public layer is strictly restricted to atomic `#![no_std]` Rust invariants and cross-language conformance harnesses.

* **Open Baseline Scope:** Pure O(1) predicate evaluation (Π(S)), deterministic poset rank precedence, and temporal/memory bounds checking.
* **Upstream Exclusion (Path C):** Hardware Abstraction Layers (HAL), eBPF Ring 0 kernel probes, seL4 integration binaries, and vaulted hardware drivers are private upstream artifacts. They are intentionally excluded from this public baseline to prevent state drift and maintain zero-egress compliance.

## System Architecture & Module Structure
This open-core repository houses the bare-metal validation primitives and Subsystem 9 specifications required to drive deterministic, fail-closed runtime security environments.

```text
src/
├── lib.rs         -> Root crate interface (#![no_std], #![deny(unsafe_code)])
├── lifecycle.rs   -> Frame bounds evaluation & dual-fault homomorphic join
└── lattice.rs     -> 5-Tier Poset Dominance Lattice Enforcement
```

## Crate Verification & Verification Posture
The live `arcstone-continuity-core` crate (v1.3.1) is a zero-dependency, `#![no_std]` predicate evaluation library. It mathematically enforces:

1. **5-Tier Poset Dominance Lattice:** `SecurityBreach (5) > Freeze (4) > Refusal (3) > LedgerCorruption (2) > Pass (1)`
2. **Dual-Fault Homomorphic Join:** Dual oversized (**S > 4096B**) and overtime (**tau > 11,990 µs**) conditions resolve to `Freeze` (Rank 4 > Rank 2) without sequential early-return latency traps.
3. **Automated CI Harness:** Verified automatically via `cargo test --lib` in `.github/workflows/substrate-ci.yml`.

## Scientific & Architectural Basis
* **WP001 (Master Anchor):** Baseline & eBPF Kernel Protection (`10.5281/zenodo.22665852`)
* **WP006 (CFN Substrate):** Bare-Metal Zero-Drag State Verification (`10.5281/zenodo.22679579`)
* **WP007 (State Reconstruction):** Deterministic Memory Boundaries (`10.5281/zenodo.22679788`)
* **WP008 (Wi-Fi 7 MLO):** Sub-12ms Transport Boundaries & Latency Clamps (`10.5281/zenodo.22680038`)
* **Canonical Specification Index:** See `docs/README.md` for full fleet listing.

### Invariant Posture
All operations comply with the zero operational drag mandate (**Cₒₚₛ = 0**). Academic staging matrices, private analytical sets, and vaulted Path C defense implementations are strictly excluded from this open distribution layer.
