# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

## Operational Parameters

* **System Baseline Release Tag:** `v1.3.1-exec`
* **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Crate Version:** `1.3.1` (Zero Dependencies)
* **Status:** FROZEN (ACTIVE)

## Architectural Scope & Public Boundary

This repository is the **Path A public/open reference surface** for the Arcstone Open-Core Substrate Membrane and its Subsystem 9 execution invariants. Path A is a downstream reference surface of the broader Arcstone architecture; it is not the complete Arcstone Computational Spine or the source of canonical system authority.

The executable surface is intentionally restricted to portable, deterministic `#![no_std]` Rust invariants and cross-language conformance material appropriate to Path A.

* **Path A Scope:** Pure O(1) predicate evaluation (Π(S)), deterministic poset rank precedence, and temporal/memory bounds checking.
* **Broader Architecture Boundary:** Arcstone specifications and whitepapers may describe additional enforcement stages, hardware realizations, formal-verification environments, private components, or governed deployment pathways outside this repository. Their inclusion in the public documentation corpus does not imply implementation or distribution by Path A.
* **Local Predicate Boundary:** `evaluate_frame_bounds()` produces a local Path A reference predicate result. Broader system or domain layers may apply additional staging, sub-status assignment, escalation, or enforcement according to their governing specifications.
* **Signal Semantics:** Raw POSIX/status numeric values are identifiers, not dominance ranks. Canonical signal dominance is defined by the explicit lattice ordering documented below.

## System Architecture & Module Structure

This repository houses the public reference validation primitives and Subsystem 9 specifications associated with deterministic, fail-closed execution environments.

```text
src/
├── lib.rs         -> Root crate interface (#![no_std], #![deny(unsafe_code)])
├── lifecycle.rs   -> Frame bounds evaluation & dual-fault homomorphic join
└── lattice.rs     -> 5-Tier Poset Dominance Lattice Enforcement
```
## Crate Verification & Verification Posture

The live `arcstone-continuity-core` crate (v1.3.1) is a zero-dependency, `#![no_std]` predicate evaluation library. Its public reference behavior defines:

1. **5-Tier Poset Dominance Lattice:** `SecurityBreach (5) > Freeze (4) > Refusal (3) > LedgerCorruption (2) > Pass (1)`
2. **Dual-Fault Homomorphic Join:** Dual oversized (**S > 4096B**) and overtime (**tau > 11,990 µs**) conditions resolve to `Freeze` (Rank 4 > Rank 2) without sequential early-return ordering.
3. **Automated Rust Verification:** The public Rust implementation is tested via `cargo test --lib` in `.github/workflows/substrate-ci.yml`.

Broader Arcstone specifications may describe additional formal-verification, kernel, hardware, or system-level assurance mechanisms. Those mechanisms are outside the executable verification boundary of this Path A repository unless explicitly included here.

## Scientific & Architectural Basis

* **WP001 (Master Anchor):** Baseline & eBPF Kernel Protection (`10.5281/zenodo.22665852`)
* **WP006 (CFN Substrate):** Bare-Metal Zero-Drag State Verification (`10.5281/zenodo.22679579`)
* **WP007 (State Reconstruction):** Deterministic Memory Boundaries (`10.5281/zenodo.22679788`)
* **WP008 (Wi-Fi 7 MLO):** Sub-12ms Transport Boundaries & Latency Clamps (`10.5281/zenodo.22680038`)
* **Canonical Specification Index:** See `docs/README.md` for full fleet listing.

### Invariant Posture

Path A preserves the public invariant posture defined for this reference surface, including **Cₒₚₛ = 0** and the declared memory/temporal bounds. Academic staging matrices, private analytical sets, hardware enforcement mechanisms, and other non-Path-A realization artifacts remain outside this open distribution layer.
