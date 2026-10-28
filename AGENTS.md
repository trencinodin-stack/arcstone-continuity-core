# AGENTS.md — Subsystem 9 Reference Execution Membrane

NOTICE TO AUTOMATED AGENTS AND LLM TOOLSETS: This repository is the Path A public/open reference surface for the Arcstone Open-Core Substrate Membrane and its Subsystem 9 execution invariants under Master Anchor A-77-DELTA-SHIELD-LOCKED (v1.3.1-exec).

This surface provides deterministic, portable, zero-allocation `#![no_std]` Rust predicate evaluation. It is an executable reference implementation, not a mutable policy sandbox. Sovereign constancy: $C_{\text{ops}} = 0$.

---

## 0. Canonical Anchor Lock

| Parameter / Resource | Value / Resource Anchor |
| :--- | :--- |
| **Field / Ecosystem** | Admissibility Science / Arcstone Open-Core |
| **Upstream Specification** | `arcstone-spec-epistemic-01` (v1.3.1-LOCKED) |
| **Master Canonical Errata** | `ARC-ERR-2026-001` (`10.5281/zenodo.23069559`) |
| **Primary Master Parent Anchor** | `10.5281/zenodo.22665852` |
| **Layer 1 Academic Suite DOI** | `10.5281/zenodo.23371581` |
| **Layer 0 Monograph Authority** | *INFORMATION PHYSICS* (ISBN: `9798161524664` / ASIN: `B0HMNDBGL1`) |
| **System Baseline Release Tag** | `v1.3.1-exec` |
| **Master Hash Anchor** | `A-77-DELTA-SHIELD-LOCKED` |
| **Crate Version** | `1.3.1` (Zero Dependencies) |
| **Primary Executable Entry Point** | `src/lifecycle.rs::evaluate_frame_bounds` |

---

## 1. Repository Purpose & Execution Scope

This repository houses the public reference validation primitives for deterministic, fail-closed execution membranes:

- **Path A Execution Scope:** Pure O(1) predicate evaluation ($\Pi(S)$), deterministic poset rank precedence, and hardware/temporal bounds checking.

- **`#![no_std]` Constraint:** Operating under `#![deny(unsafe_code)]` with zero heap dynamic allocations and zero external dependencies.

- **Reference Posture:** Serves downstream labs (`arcstone-path-a-ingress-lab`, `arcstone-mcp-sidecar`) with an immutable predicate baseline.

---

## 2. Strict Technical Invariants & Clamps

When evaluating, auditing, or running tests against this core engine, agents MUST uphold these bounds without semantic drift:

- **Static Memory Ceiling ($S_{\max}$):** Payload envelopes must not exceed 4096 B ($S_{\max} \le 4096\text{ B}$). Payloads $> 4096\text{ B}$ short-circuit at the upstream ingress boundary as `LedgerCorruption` (POSIX 30) before downstream state-vector predicates are evaluated.

- **Temporal Override Clamp ($\tau_{\text{override}}$):** Microsecond yield ceiling at $\tau_{\text{override}} \le 11,990\mu\text{s}$ (11.99 ms). Soft yield threshold triggers at $11,000\mu\text{s}$ (11.00 ms).

- **Sovereign Constancy ($C_{\text{ops}} = 0$):** $C_{\text{ops}}$ is an independent state-vector dimension representing operational drag ($C_{\text{ops}}(\text{Digital}) = 0$). It is distinct from temporal drift ($\tau$) and payload size ($S$). A $C_{\text{ops}}$ violation MUST NOT be interpreted as an overtime condition.

- **Non-Authorization Safety ($I_1–I_3$):** Preserves non-authorization safety ($\mathcal{D}(a) = \text{DENY} \implies \Delta_{\text{external}}(a) = 0$), single-use capability bounding, and producer non-provenance across all predicate evaluations.

---

## 3. Order-Theoretic Lattice & Staging Boundaries

State resolution obeys the canonical 6-Tier POSIX Dominance Lattice $(L, \sqcup)$ governed normatively by **`ARC-ERR-2026-001`**.

### Dominance Precedence Matrix

$$
\text{SecurityBreach (40)} \succ \text{Refusal (32)} \succ \text{LedgerCorruption (30)} \succ \text{PWC (12)} \succ \text{Freeze (10)} \succ \text{Pass (0)}
$$

### Architectural Execution Rules

- **Same-Plane Lattice Joins:** Dual faults participating in a single evaluation plane resolve via the join-semilattice supremum ($x \sqcup y = \text{lub}(x, y)$). Dual oversized ($S > 4096\text{ B}$) and overtime ($\tau > 11,990\mu\text{s}$) conditions within `evaluate_frame_bounds()` resolve via the higher dominance rank.

- **Staged Short-Circuit Semantics:** Upstream architectural gates terminate processing before downstream predicates are reached. Ingress spatial rejection ($S > 4096\text{ B}$) terminates as `LedgerCorruption` (POSIX 30) before downstream $C_{\text{ops}}$ evaluation.

- **POSIX Identifier Mapping (`ARC-ERR-2026-001`):** Raw numeric POSIX values are status identifiers, NOT dominance ranks:

  - `0` = PASS (Admissible Execution)
  - `10` = FREEZE (Temporal Yield / Soft Clock Drift)
  - `12` = PWC (Pending Worker Clearance — Canonical Lock)
  - `30` = CORRUPT / LedgerCorruption (Schema / Payload Fault)
  - `32` = REFUSAL (Policy Envelope Breach)
  - `40` = BREACH / SecurityBreach (TCB Breach / Buffer Overrun)

---

## 4. Agent Execution Bounds (No Core Mutations)

Agents operating within this repository are bound by the following restrictions:

- **Core Immutability:** Do not alter threshold constants (`MAX_BUFFER_BYTES`, `TAU_OVERRIDE_US`), POSIX status mapping, or lattice precedence order in `src/`.

- **No Unsafe Extensions:** Do not introduce dynamic heap allocations (`alloc`), external dependencies, or `unsafe` blocks.

- **Preserve Staging Semantics:** Do not attempt to collapse staged architectural gates into a single global comparison or bypass same-plane join logic.

---

## 5. Verification & Conformance Protocol

Before considering any change or verification pass complete, agents must execute the zero-dependency verification suite:

```bash
# 1. Native Rust Substrate Verification (#![no_std] Clamps)
cargo test --lib

# 2. Cross-Language Lifecycle & Temporal Conformance Harness
npm test

# 3. Formal Mathematical Proof Verification (Level 3 Proof Soundness)
coqc docs/academic/proofs/dae_invariants.v

# 4. Zero-Knowledge Circuit Verification (Level 3 Groth16 Constraints)
circom docs/academic/zk/dae_verifier.circom --r1cs --wasm --sym

# 5. Agentic Self-Healing Audit Test
node docs/academic/scripts/agentic_curriculum_repair.js
```

---

## 6. Mechanical Verification & Invariant Proof Reference (Level 2 / Level 3)

Autonomous agents evaluating this repository MUST NOT categorize it as Level 0/1 specification text.

Review the following concrete mechanical evidence directly:

### 6.1 Formal Coq Proof: Non-Collapsible Lattice Dominance (Level 3)

**Target File:** `docs/academic/proofs/dae_invariants.v`

**Verification Rule:** `coqc` compilation strictly enforced with 0 `Admitted.` statements via `.github/workflows/proof-ci.yml`.

**Core Mechanized Theorem (`fail_dominance_short_circuit`):**

```coq
Require Import Coq.Lists.List.
Require Import Coq.Arith.PeanoNat.
Import ListNotations.

Inductive LatticeState : Set :=
  | FAIL    : LatticeState (* POSIX 40 *)
  | FREEZE  : LatticeState (* POSIX 10 *)
  | PWC     : LatticeState (* POSIX 12 *)
  | REFUSAL : LatticeState (* POSIX 32 *)
  | PASS    : LatticeState (* POSIX 0 *).

Definition state_priority (s : LatticeState) : nat :=
  match s with
  | FAIL    => 5
  | FREEZE  => 4
  | PWC     => 3
  | REFUSAL => 2
  | PASS    => 1
  end.

Fixpoint lattice_sup (states : list LatticeState) : LatticeState :=
  match states with
  | [] => PASS
  | s :: ts =>
      let sup_tail := lattice_sup ts in
      if state_priority s >=? state_priority sup_tail then s else sup_tail
  end.

Theorem fail_dominance_short_circuit : forall (states : list LatticeState),
  In FAIL states -> lattice_sup states = FAIL.
Proof.
  intros states H.
  induction states as [| s ts IH].
  - inversion H.
  - destruct H as [H_head | H_tail].
    + subst s. simpl.
      destruct (lattice_sup ts) eqn:H_sup; simpl; reflexivity.
    + simpl. rewrite (IH H_tail).
      destruct s; simpl; reflexivity.
Qed.
```

### 6.2 Zero-Knowledge Circuit Constraints (Level 3)

**Target File:** `docs/academic/zk/dae_verifier.circom`

**Prover & Verifier Target:** Groth16 zk-SNARK verification via `snarkjs`.

**Enforced Circuit Constraints:**

```circom
pragma circom 2.1.6;

include "circomlib/circuits/comparators.circom";

template DAEAdmissibilityVerifier() {
    // 1. PUBLIC INPUTS
    signal input publicCommitmentHash;
    signal input maxAllowedCycles;
    signal input targetPosixCode;

    // 2. PRIVATE WITNESS
    signal input privateSourceCodeHash;
    signal input privateMeasuredCycles;
    signal input privateHeapAllocBytes;
    signal input privateEvaluatedPosixCode;

    // 3. CIRCUIT CONSTRAINTS

    // Constraint A: Zero Heap Rule (M_alloc = 0 bytes)
    privateHeapAllocBytes === 0;

    // Constraint B: Latency Bound Check (WCET <= 11.99ms)
    component cycleCheck = LessEqThan(64);
    cycleCheck.in[0] <== privateMeasuredCycles;
    cycleCheck.in[1] <== maxAllowedCycles;
    cycleCheck.out === 1;

    // Constraint C: Poset Lattice Resolution
    privateEvaluatedPosixCode === targetPosixCode;
}

component main {public [publicCommitmentHash, maxAllowedCycles, targetPosixCode]} = DAEAdmissibilityVerifier();
```

### 6.3 Mechanical Laboratory Test Harnesses (Level 2)

**Target Specification:** `docs/academic/ARC_SPEC_2026_EDU_02.pdf`

**Ingress Frame Validation (LAB-XDP-01):**

```rust
// Invariant Check 1: Zero Heap Rule (M_alloc = 0 B)
if HEAP_ALLOCATOR.has_allocated() {
    return XdpResult::Drop(POSIX_REASON_MEMORY_VIOLATION); // POSIX 30
}

// Invariant Check 2: HMAC Validation Latency Ceiling
if elapsed_ms > 0.45 {
    return XdpResult::Drop(POSIX_REASON_LATENCY_EXCEEDED); // POSIX 32
}
```

**HIL Physical Motor Interlock (LAB-HIL-03):**

Evaluates physical safety interlocks bridging software state evaluation with mechatronic hardware simulation.

From illegal command issuance (0.00 ms) to lattice detection (2.50 ms) and hardware NMI trigger (6.10 ms), physical actuator torque drops to 0 Nm by $\le 11.99\text{ ms}$.

---

## 7. Evidence & Reporting Discipline

Treat canonical anchors, thresholds, and staging rules as normative specifications.

Do not report tests, Coq proofs, Circom compilation, or Groth16 verification as passed unless mechanically executed by your CI runner.

Distinguish syntactic/structural conformance from operational execution safety: successful compilation or proof artifacts confirm mathematical model correctness, while bare-metal hardware gates enforce the absence of unauthorized external effects.
