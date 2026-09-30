# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

[![Release](https://img.shields.io/badge/release-v1.3.1--exec-blue.svg)](https://github.com/trencinodin-stack/arcstone-continuity-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg)](LICENSE)
[![Build](https://img.shields.io/badge/no__std-compatible-orange.svg)]()
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.22966979.svg)](https://doi.org/10.5281/zenodo.22966979)

> 📌 **Normative Specification & Errata Notice (`ARC-ERR-2026-001`)**  
> All runtime implementations and predicate evaluations in `arcstone-continuity-core` conform strictly to **[ARC-ERR-2026-001](https://doi.org/10.5281/zenodo.23069559)** (*Master Canonical Errata, Disambiguation & Refinement Protocol*).  
> **Primary Suite Anchor:** [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852) | **Master Hash:** `A-77-DELTA-SHIELD-LOCKED`  
> **Canonical Signal Interface:** `POSIX 0` (PASS) | `POSIX 10` (FREEZE) | `POSIX 12` (PWC) | `POSIX 30` (CORRUPT) | `POSIX 32` (REFUSAL) | `POSIX 40` (BREACH)

## Operational Parameters

* **System Baseline Release Tag:** `v1.3.1-exec`
* **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Crate Version:** `1.3.1` (Zero Dependencies)
* **Status:** FROZEN (ACTIVE)

## Architectural Scope & Public Boundary

This repository is the **Path A public/open reference surface** for the Arcstone Open-Core Substrate Membrane and its Subsystem 9 execution invariants. Path A is a downstream reference surface of the broader Arcstone architecture; it is not the complete Arcstone Computational Spine or the source of canonical system authority.

The executable surface is intentionally restricted to portable, deterministic `#![no_std]` Rust invariants and cross-language conformance material appropriate to Path A.

* **Path A Scope:** Pure $O(1)$ predicate evaluation ($\Pi(S)$), deterministic poset rank precedence, and temporal/memory bounds checking.
* **Broader Architecture Boundary:** Arcstone specifications and whitepapers may describe additional enforcement stages, hardware realizations, formal-verification environments, private components, or governed deployment pathways outside this repository. Their inclusion in the public documentation corpus does not imply implementation or distribution by Path A.
* **Local Predicate Boundary:** `evaluate_frame_bounds()` produces a local Path A reference predicate result. Broader system or domain layers may apply additional staging, sub-status assignment, escalation, or enforcement according to their governing specifications.
* **Architectural Staging Boundary:** The broader architecture contains intentionally ordered enforcement stages. An upstream short-circuit terminates processing before predicates assigned to downstream stages are evaluated. Canonical dominance therefore applies to faults participating in the same evaluation plane; it does not retroactively override a result already terminated by an earlier architectural gate.
* **Ingress Size Short-Circuit:** An oversized payload ($S > 4096\text{B}$) is rejected at the upstream ingress boundary with `LedgerCorruption` / POSIX 30 before downstream state-vector predicates such as $C_{\text{ops}}$ are evaluated. Accordingly, a conceptual input containing both $S > 4096\text{B}$ and $C_{\text{ops}} \neq 0$ resolves to POSIX 30 at that ingress boundary because processing terminates before the downstream $C_{\text{ops}}$ predicate is reached.
* **Sovereign Constancy Boundary:** $C_{\text{ops}}$ is an independent state-vector dimension representing the Sovereign Constancy invariant ($C_{\text{ops}} = 0$). It is distinct from temporal drift ($\tau$) and from the payload-size bound ($S$). A $C_{\text{ops}}$ violation must not be interpreted as equivalent to an overtime or clock-drift condition.
* **Signal Semantics:** Raw POSIX/status numeric values are identifiers, not dominance ranks. Canonical signal dominance is defined by the explicit lattice ordering documented below.
* **Dominance Scope:** Canonical lattice precedence governs competing statuses within a common evaluation plane. It does not collapse intentionally sequential architectural gates into a single global comparison.

## System Architecture & Module Structure

This repository houses the public reference validation primitives and Subsystem 9 specifications associated with deterministic, fail-closed execution environments.

```text
src/
├── lib.rs        -> Root crate interface (#![no_std], #![deny(unsafe_code)])
├── lifecycle.rs  -> Frame bounds evaluation & dual-fault homomorphic join
└── lattice.rs    -> 5-Tier Poset Dominance Lattice Enforcement
```
The Rust frame-bounds predicate and the broader staged architecture should not be interpreted as the same execution operation. `evaluate_frame_bounds()` demonstrates local Path A lattice behavior for the predicates assigned to that reference operation. Architectural ingress short-circuiting and downstream state-vector evaluation remain distinct where specified by the governing architecture.

### Crate Verification & Verification Posture

The live `arcstone-continuity-core` crate (v1.3.1) is a zero-dependency, `#![no_std]` predicate evaluation library. Its public reference behavior defines:

- **5-Tier Poset Dominance Lattice:** `SecurityBreach` (POSIX 40) > `Freeze` (POSIX 10) > `Refusal` (POSIX 32) > `LedgerCorruption` (POSIX 30) > `Pass` (POSIX 0)
- **Dual-Fault Homomorphic Join:** Within `evaluate_frame_bounds()`, dual oversized ($S > 4096\text{B}$) and overtime ($\tau > 11,990\mu\text{s}$) conditions resolve to `Freeze` (Rank 4 > Rank 2) because both predicates participate in that local evaluation operation.
- **Staged Short-Circuit Semantics:** The broader architecture may terminate an input at an earlier enforcement stage before a downstream predicate is evaluated. In particular, the ingress $S > 4096\text{B}$ boundary resolves to `LedgerCorruption` / POSIX 30 before downstream $C_{\text{ops}}$ evaluation. This staged result is not a violation of the same-plane dominance lattice.
- **Automated Rust Verification:** The public Rust implementation is tested via `cargo test --lib` in `.github/workflows/substrate-ci.yml`.
- **Cross-Language Conformance:** TypeScript conformance material preserves the staged and status-mapping semantics applicable to the public Path A reference surface.

Broader Arcstone specifications may describe additional formal-verification, kernel, hardware, or system-level assurance mechanisms. Those mechanisms are outside the executable verification boundary of this Path A repository unless explicitly included here.

## Scientific & Architectural Basis

* **Master Errata Protocol:** [ARC-ERR-2026-001: Master Canonical Errata, Disambiguation & Refinement Protocol](https://doi.org/10.5281/zenodo.23069559)
* **ARC-PUB-2026-CORE01:** [Arcstone Continuity Core: Fail-Closed Execution Membrane](https://doi.org/10.5281/zenodo.22966979) *(Primary Crate Anchor)*
* **WP001 (Master Suite Anchor):** [Baseline & eBPF Kernel Protection](https://doi.org/10.5281/zenodo.22665852)
* **WP006 (CFN Substrate):** [Bare-Metal Zero-Drag State Verification](https://doi.org/10.5281/zenodo.22679579)
* **WP007 (State Reconstruction):** [Deterministic Memory Boundaries](https://doi.org/10.5281/zenodo.22679788)
* **WP008 (Wi-Fi 7 MLO):** [Sub-12ms Transport Boundaries & Latency Clamps](https://doi.org/10.5281/zenodo.22680038)

## Invariant Posture

Path A preserves the public invariant posture defined for this reference surface, including $C_{\text{ops}} = 0$ and the declared memory/temporal bounds.

For interpretation of these invariants, three boundaries remain distinct:

1. **Architectural staging** determines which predicates are reached and which earlier gates may terminate processing.
2. **Local predicate evaluation** determines the result of the predicates participating in a particular Path A reference operation.
3. **Canonical dominance** determines precedence among competing statuses within the same applicable evaluation plane.

These distinctions prevent raw status identifiers, sequential architectural gates, and local lattice operations from being incorrectly treated as interchangeable semantics.
