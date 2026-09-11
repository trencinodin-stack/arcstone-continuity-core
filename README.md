Markdown
# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

## Operational Parameters
* **System Baseline Release Tag:** `v1.3.0-exec`
* **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Status:** FROZEN (ACTIVE)

## Architectural Scope & Public Boundary
This repository represents the open-core reference baseline for Subsystem 9 execution membranes. To maintain zero operational drag (*C*<sub>ops</sub> = 0) and absolute memory bounds (*S*<sub>max</sub> ≤ 4096B), this public layer is strictly restricted to atomic `#![no_std]` Rust invariants and cross-language conformance harnesses.

* **Open Baseline Scope:** Pure *O*(1) predicate evaluation (Π(*S*)), deterministic status precedence, and temporal/memory bounds checking.
* **Upstream Exclusion (Path C):** Hardware Abstraction Layers (HAL), eBPF Ring 0 kernel probes, seL4 integration binaries, and vaulted hardware drivers are private upstream artifacts. They are intentionally excluded from this public baseline to prevent state drift and maintain zero-egress compliance.

## Architecture Overview
This open-core repository houses the bare-metal validation primitives and Subsystem 9 specifications required to drive deterministic, fail-closed runtime security environments.

18  ```text
19  src/
20  ├── lib.rs          -> Hardware envelope declarations (S_max <= 4096B)
21  ├── lifecycle.rs    -> Temporal ceiling pre-filtering (tau_override <= 11.99ms)
22  └── lattice.rs      -> 5-Tier Poset Dominance Lattice Enforcement
23  ```
24  
25  ## Scientific & Architectural Basis

WP001 (Master Anchor): Baseline & eBPF Kernel Protection (10.5281/zenodo.22665852)

WP006 (CFN Substrate): Bare-Metal Zero-Drag State Verification (10.5281/zenodo.22679579)

WP007 (State Reconstruction): Deterministic Memory Boundaries (10.5281/zenodo.22679788)

WP008 (Wi-Fi 7 MLO): Sub-12ms Transport Boundaries & Latency Clamps (10.5281/zenodo.22680038)

Canonical Specification Index: See docs/README.md for full fleet listing.

Invariant Posture
All operations comply with the zero operational drag mandate (Cops = 0). Academic staging matrices, private analytical sets, and vaulted Path C defense implementations are strictly excluded from this open distribution layer.
