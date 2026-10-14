# Arcstone Open-Core Substrate Membrane (`arcstone-continuity-core`)

[![Release](https://img.shields.io/badge/release-v1.3.1--exec-blue.svg)](https://github.com/trencinodin-stack/arcstone-continuity-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg)](LICENSE)
[![Build](https://img.shields.io/badge/no__std-compatible-orange.svg)]()
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23371581.svg)](https://doi.org/10.5281/zenodo.23371581)

> 📌 **Normative Specification & Errata Notice (`ARC-ERR-2026-001`)**  
> All runtime implementations and predicate evaluations in `arcstone-continuity-core` conform strictly to **[ARC-ERR-2026-001](https://doi.org/10.5281/zenodo.23069559)** (*Master Canonical Errata, Disambiguation & Refinement Protocol*).  
> **Primary Suite Anchor:** [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852) | **Layer 1 Master Academic Suite DOI:** [10.5281/zenodo.23371581](https://doi.org/10.5281/zenodo.23371581)  
> **Layer 0 Monograph Authority:** *INFORMATION PHYSICS* (ISBN: `9798161524664` / ASIN: [`B0HMNDBGL1`](https://www.amazon.com/dp/B0HMNDBGL1)) | **Master Hash:** `A-77-DELTA-SHIELD-LOCKED`  
> **Canonical Signal Interface:** `POSIX 0` (PASS) | `POSIX 10` (FREEZE) | `POSIX 12` (PWC) | `POSIX 30` (CORRUPT) | `POSIX 32` (REFUSAL) | `POSIX 40` (BREACH)

## Operational Parameters

- **System Baseline Release Tag:** `v1.3.1-exec`
- **Target Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
- **Crate Version:** `1.3.1` (Zero Dependencies)
- **Status:** FROZEN (ACTIVE)

---

## Architectural Scope & Public Boundary

This repository is the **Path A public/open reference surface** for the Arcstone Open-Core Substrate Membrane and its Subsystem 9 execution invariants. Path A is a downstream reference surface of the broader Arcstone architecture; it is not the complete Arcstone Computational Spine or the source of canonical system authority.

The executable surface is intentionally restricted to portable, deterministic `#![no_std]` Rust invariants and cross-language conformance material appropriate to Path A.

- **Path A Scope:** Pure $O(1)$ predicate evaluation ($\Pi(S)$), deterministic poset rank precedence, and temporal/memory bounds checking.
- **Broader Architecture Boundary:** Arcstone specifications and whitepapers may describe additional enforcement stages, hardware realizations, formal-verification environments, private components, or governed deployment pathways outside this repository. Their inclusion in the public documentation corpus does not imply implementation or distribution by Path A.
- **Local Predicate Boundary:** `evaluate_frame_bounds()` produces a local Path A reference predicate result. Broader system or domain layers may apply additional staging, sub-status assignment, escalation, or enforcement according to their governing specifications.
- **Architectural Staging Boundary:** The broader architecture contains intentionally ordered enforcement stages. An upstream short-circuit terminates processing before predicates assigned to downstream stages are evaluated. Canonical dominance therefore applies to faults participating in the same evaluation plane; it does not retroactively override a result already terminated by an earlier architectural gate.
- **Ingress Size Short-Circuit:** An oversized payload ($S > 4096\text{B}$) is rejected at the upstream ingress boundary with `LedgerCorruption` / POSIX 30 before downstream state-vector predicates such as $C_{\text{ops}}$ are evaluated. Accordingly, a conceptual input containing both $S > 4096\text{B}$ and $C_{\text{ops}} \neq 0$ resolves to POSIX 30 at that ingress boundary because processing terminates before the downstream $C_{\text{ops}}$ predicate is reached.
- **Sovereign Constancy Boundary:** $C_{\text{ops}}$ is an independent state-vector dimension representing the Sovereign Constancy invariant ($C_{\text{ops}} = 0$). It is distinct from temporal drift ($\tau$) and from the payload-size bound ($S$). A $C_{\text{ops}}$ violation must not be interpreted as equivalent to an overtime or clock-drift condition.
- **Signal Semantics:** Raw POSIX/status numeric values are identifiers, not dominance ranks. Canonical signal dominance is defined by the explicit lattice ordering documented below.
- **Dominance Scope:** Canonical lattice precedence governs competing statuses within a common evaluation plane. It does not collapse intentionally sequential architectural gates into a single global comparison.

---

## System Architecture & Module Structure

This repository houses the public reference validation primitives, Subsystem 9 specifications, formal Coq verification proofs, and zero-knowledge circuits associated with deterministic, fail-closed execution environments.

```text
src/
├── lib.rs          -> Root crate interface (#![no_std], #![deny(unsafe_code)])
├── lifecycle.rs    -> Frame bounds evaluation & dual-fault homomorphic join
└── lattice.rs      -> 5-Tier Poset Dominance Lattice Enforcement

docs/academic/
├── 00_Master_Architectural_Index_DAE.pdf     # Primary Citation Anchor & Master Map
├── ARC-ERR-2026-001.pdf                     # Canonical POSIX Code 12 Lock Errata
├── ARC-SPEC-2026-EDU-01.pdf                 # Foundations & Poset Lattice Algebra
├── ARC-SPEC-2026-EDU-02.pdf                 # Bare-Metal Laboratory Harness Suite
├── ARC-SPEC-2026-EDU-03.pdf                 # Industrial Accreditation & ABET (SO 1-7)
├── ARC-SPEC-2026-EDU-04.pdf                 # AI Prompt Grounding & Pedagogy Schema
├── ARC-SPEC-2026-EDU-05.pdf                 # Coq Formal Proofs & Mechanical Soundness
├── ARC-SPEC-2026-EDU-06.pdf                 # Physical Mechatronic & Aerospace Transduction
├── ARC-SPEC-2026-EDU-07.pdf                 # Zero-Knowledge Cryptography (Circom)
├── ARC-SPEC-2026-EDU-08.pdf                 # Autonomous Agentic Self-Healing Pedagogy
├── ARC-SPEC-2026-EDU-09.pdf                 # Universal Systems Transduction (Bio/Finance)
├── ARC-SPEC-2026-EXEC-01.pdf                # Dual Launch Execution Protocol
├── proofs/
│   └── dae_invariants.v                     # Coq Formal Verification Proofs
├── schema/
│   └── dae_pedagogy_v1.json                 # Machine-Readable AI Grounding Manifest
├── scripts/
│   └── agentic_curriculum_repair.js         # ACEP Agentic Self-Healing Repair Engine
└── zk/
    └── dae_verifier.circom                  # Groth16 zk-SNARK Admissibility Circuit
