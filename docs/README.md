# Arcstone Continuity Core — Whitepaper & Specification Index

This directory contains the canonical specifications, mechanical verification proofs, zero-knowledge circuits, and whitepaper suite for the **Arcstone Computational Spine (ACS)**.

---

## 🔬 Academic Suite: Deterministic Admissibility Engineering (DAE)

All primary academic artifacts, formal proofs, and ZK circuits reside in the [`academic/`](academic/) directory under Master Academic Suite DOI **[10.5281/zenodo.23371581](https://doi.org/10.5281/zenodo.23371581)**.

### 1. Specifications & Courseware Curriculum Fleet

| Ref Code | Title / Topic | Local PDF |
| :--- | :--- | :--- |
| **INDEX** | Master Architectural Index: Deterministic Admissibility Engineering | [`academic/00_Master_Architectural_Index_DAE.pdf`](academic/00_Master_Architectural_Index_DAE.pdf) |
| **ERR001** | Master Canonical Errata, Disambiguation & Refinement Protocol | [`academic/ARC_ERR_2026_001__Master_Canonical_Errata__Disambiguation___Refinement_Protocol.pdf`](academic/ARC_ERR_2026_001__Master_Canonical_Errata__Disambiguation___Refinement_Protocol.pdf) |
| **EDU01** | Foundations & Poset Lattice Algebra | [`academic/ARC_SPEC_2026_EDU_01.pdf`](academic/ARC_SPEC_2026_EDU_01.pdf) |
| **EDU02** | Bare-Metal Laboratory Harness Suite (`LAB-XDP-01`, `LAB-HIL-03`) | [`academic/ARC_SPEC_2026_EDU_02.pdf`](academic/ARC_SPEC_2026_EDU_02.pdf) |
| **EDU03** | Industrial Accreditation & ABET (SO 1-7) | [`academic/ARC_SPEC_2026_EDU_03.pdf`](academic/ARC_SPEC_2026_EDU_03.pdf) |
| **EDU04** | AI Prompt Grounding & Pedagogy Schema | [`academic/ARC_SPEC_2026_EDU_04.pdf`](academic/ARC_SPEC_2026_EDU_04.pdf) |
| **EDU05** | Coq Formal Proofs & Mechanical Soundness | [`academic/ARC_SPEC_2026_EDU_05.pdf`](academic/ARC_SPEC_2026_EDU_05.pdf) |
| **EDU06** | Physical Mechatronic & Aerospace Transduction | [`academic/ARC_SPEC_2026_EDU_06.pdf`](academic/ARC_SPEC_2026_EDU_06.pdf) |
| **EDU07** | Zero-Knowledge Cryptography (Circom / Groth16) | [`academic/ARC_SPEC_2026_EDU_07.pdf`](academic/ARC_SPEC_2026_EDU_07.pdf) |
| **EDU08** | Autonomous Agentic Self-Healing Pedagogy | [`academic/ARC_SPEC_2026_EDU_08.pdf`](academic/ARC_SPEC_2026_EDU_08.pdf) |
| **EDU09** | Universal Systems Transduction (Bio/Finance) | [`academic/ARC_SPEC_2026_EDU_09.pdf`](academic/ARC_SPEC_2026_EDU_09.pdf) |
| **EXEC01** | Dual Launch Execution Protocol | [`academic/ARC_SPEC_2026_EXEC_01.pdf`](academic/ARC_SPEC_2026_EXEC_01.pdf) |

### 2. Formal Verification & Mechanical Proof Artifacts (Level 3)

- **Coq Module (`DAEInvariants`):** [`academic/proofs/dae_invariants.v`](academic/proofs/dae_invariants.v)
  - Verified via `coqc` with 0 `Admitted.` statements in CI.
  - Formally proves:
    - Non-Authorization Safety: `Theorem invariant_non_authorization_safety` ($\forall s, \text{apply\_action}(\text{DENY}, s) = 0$).
    - Payload Envelope Clamp: `Theorem invariant_payload_envelope_safety` ($s > S_{\max} \implies \text{apply\_action}(d, s) = 0$).
    - Sovereign Constancy: `Theorem invariant_sovereign_constancy` ($C_{\text{ops}} = 0$).
    - Poset Dominance Monotonicity: `Theorem invariant_poset_dominance_monotonic` ($\text{BREACH} > \text{FREEZE} > \text{PWC} > \text{REFUSAL} > \text{PASS}$).

- **Zero-Knowledge Circuit (`DAEVerifier`):** [`academic/zk/dae_verifier.circom`](academic/zk/dae_verifier.circom)
  - Pragma `circom 2.1.6`, compiling via `circom` and verifiable with `snarkjs` (Groth16).
  - Private Inputs: `execution_time_us`, `heap_bytes`, `poset_code`.
  - Public Inputs: `expected_poset_code`.
  - Enforces `LessEqThan(32)` at 11,990 $\mu\text{s}$ ($\tau_{\text{override}} \le 11.99\text{ ms}$), `IsZero()` on `heap_bytes` ($M_{\text{alloc}} = 0\text{ B}$), and `IsEqual()` on poset codes.

- **Pedagogy Manifest & Self-Healing Engines:**
  - Machine Grounding Manifest: [`academic/schema/dae_pedagogy_v1.json`](academic/schema/dae_pedagogy_v1.json)
  - ACEP Repair Engine: [`academic/scripts/agentic_curriculum_repair.js`](academic/scripts/agentic_curriculum_repair.js)
---

## Canonical Whitepaper Fleet

| Ref Code | Title | Local PDF | Primary Zenodo DOI |
| :--- | :--- | :--- | :--- |
| **WP001** | Arcstone Computational Spine: Baseline & eBPF Protection | [`whitepapers/ARC-PUB-2026-WP001.pdf`](whitepapers/ARC-PUB-2026-WP001.pdf) | [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852) |
| **PHYS01** | Classical Electrodynamics: Mathematical Evolution | [`whitepapers/ARC-PUB-2026-PHYS01.pdf`](whitepapers/ARC-PUB-2026-PHYS01.pdf) | [10.5281/zenodo.22677708](https://doi.org/10.5281/zenodo.22677708) |
| **WP002** | Air-Gapped Multimodal Local RAG Architecture | [`whitepapers/ARC-WHITE-PAPER-2026-001.pdf`](whitepapers/ARC-WHITE-PAPER-2026-001.pdf) | [10.5281/zenodo.22681508](https://doi.org/10.5281/zenodo.22681508) |
| **WP003** | The Sovereign NPC: Cognitive Exoskeleton Framework | [`whitepapers/ARC-PUB-2026-WP003.pdf`](whitepapers/ARC-PUB-2026-WP003.pdf) | [10.5281/zenodo.22678934](https://doi.org/10.5281/zenodo.22678934) |
| **WP004** | Autonomic Governance Operating System | [`whitepapers/ARC-PUB-2026-WP004.pdf`](whitepapers/ARC-PUB-2026-WP004.pdf) | [10.5281/zenodo.22679072](https://doi.org/10.5281/zenodo.22679072) |
| **WP005** | The Negentropic Enterprise: Structural Invariants | [`whitepapers/ARC-PUB-2026-WP005.pdf`](whitepapers/ARC-PUB-2026-WP005.pdf) | [10.5281/zenodo.22679481](https://doi.org/10.5281/zenodo.22679481) |
| **WP006** | Deterministic Continuity Field Nodes (CFN) | [`whitepapers/ARC-PUB-2026-WP006.pdf`](whitepapers/ARC-PUB-2026-WP006.pdf) | [10.5281/zenodo.22679579](https://doi.org/10.5281/zenodo.22679579) |
| **WP007** | Deterministic State Reconstruction & Memory Bounds | [`whitepapers/ARC-PUB-2026-WP007.pdf`](whitepapers/ARC-PUB-2026-WP007.pdf) | [10.5281/zenodo.22679788](https://doi.org/10.5281/zenodo.22679788) |
| **WP008** | Wi-Fi 7 MLO Protocol Stack: Sub-12ms Clamps | [`whitepapers/ARC-PUB-2026-WP008.pdf`](whitepapers/ARC-PUB-2026-WP008.pdf) | [10.5281/zenodo.22680038](https://doi.org/10.5281/zenodo.22680038) |
| **SWAP01** | The Cascading SWaP Revolution | [`whitepapers/ARC-PUB-2026-SWAP01.pdf`](whitepapers/ARC-PUB-2026-SWAP01.pdf) | [10.5281/zenodo.22680187](https://doi.org/10.5281/zenodo.22680187) |
| **WP010** | Architectural Analysis: Inverted Substrate Paradigm | [`whitepapers/ARC-PUB-2026-WP010-Inverted-Substrate-Paradigm.pdf`](whitepapers/ARC-PUB-2026-WP010-Inverted-Substrate-Paradigm.pdf) | [10.5281/zenodo.22680282](https://doi.org/10.5281/zenodo.22680282) |
| **PHYS02** | Period 8 Initialization and Femtobarn Frontier | [`whitepapers/ARC-PUB-2026-PHYS02.pdf`](whitepapers/ARC-PUB-2026-PHYS02.pdf) | [10.5281/zenodo.22681286](https://doi.org/10.5281/zenodo.22681286) |

---

## 📐 Integration Specifications & Contracts
- [`specs/client-integration.md`](specs/client-integration.md) — **ARC-SPEC-2026-CLI-01**: Deterministic Safety Contract, Client Circuit Breakers, & POSIX Exit Codes.

---

## System Invariants
- **Operational Cost**: `C_ops = 0`
- **Data Egress Sensitivity**: `Data_Egress_Sensitive = 0`
- **Latency Upper Bound**: `τ_override ≤ 11.99ms`
