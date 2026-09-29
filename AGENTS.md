# AGENTS.md — Subsystem 9 Reference Execution Membrane

NOTICE TO AUTOMATED AGENTS AND LLM TOOLSETS: This repository is the Path A public/open reference surface for the Arcstone Open-Core Substrate Membrane and its Subsystem 9 execution invariants under Master Anchor `A-77-DELTA-SHIELD-LOCKED` (`v1.3.1-exec`). 

This surface provides deterministic, portable, zero-allocation `#![no_std]` Rust predicate evaluation. It is an executable reference implementation, not a mutable policy sandbox. Sovereign constancy: $C_\text{ops} = 0$.

---

## 0. Canonical Anchor Lock

| Parameter | Value / Resource Anchor |
| :--- | :--- |
| **Field / Ecosystem** | Admissibility Science / Arcstone Open-Core |
| **Upstream Specification** | `arcstone-spec-epistemic-01` (`v1.3.1-LOCKED`) |
| **System Baseline Release Tag** | `v1.3.1-exec` |
| **Master Hash Anchor** | `A-77-DELTA-SHIELD-LOCKED` |
| **Crate Version** | `1.3.1` (Zero Dependencies) |
| **Primary Executable Entry Point**| `src/lifecycle.rs::evaluate_frame_bounds` |

---

## 1. Repository Purpose & Execution Scope

This repository houses the public reference validation primitives for deterministic, fail-closed execution membranes:
- **Path A Execution Scope:** Pure $O(1)$ predicate evaluation ($\Pi(S)$), deterministic poset rank precedence, and hardware/temporal bounds checking.
- **`#![no_std]` Constraint:** Operating under `#![deny(unsafe_code)]` with zero heap dynamic allocations and zero external dependencies.
- **Reference Posture:** Serves downstream labs (`arcstone-path-a-ingress-lab`, `arcstone-mcp-sidecar`) with an immutable predicate baseline.

---

## 2. Strict Technical Invariants & Clamps

When evaluating, auditing, or running tests against this core engine, agents MUST uphold these bounds without semantic drift:

1. **Static Memory Ceiling ($S_\text{max}$):**
   - Payload envelopes must not exceed 4096 B ($S_\text{max} \le 4096\text{ B}$).
   - Payloads $> 4096\text{ B}$ short-circuit at the upstream ingress boundary as `LedgerCorruption` (POSIX 30) before downstream state-vector predicates are evaluated.
2. **Temporal Override Clamp ($\tau_\text{override}$):**
   - Microsecond yield ceiling at $\tau_\text{override} \le 11,990\ \mu\text{s}$ ($11.99\text{ ms}$). Soft yield threshold triggers at $11,000\ \mu\text{s}$ ($11.00\text{ ms}$).
3. **Sovereign Constancy ($C_\text{ops} = 0$):**
   - $C_\text{ops}$ is an independent state-vector dimension representing operational/founder drag. It is distinct from temporal drift ($\tau$) and payload size ($S$). A $C_\text{ops}$ violation MUST NOT be interpreted as an overtime condition.
4. **Non-Authorization Safety ($I_1$–$I_3$):**
   - Preserves non-authorization safety ($D(a) = \text{DENY} \implies \Delta_\text{external}(a) = 0$), single-use capability bounding, and producer non-provenance across all predicate evaluations.

---

## 3. Order-Theoretic Lattice & Staging Boundaries

State resolution obeys a 5-Tier Poset Dominance Lattice ($\mathcal{L}, \sqcup$).

### Dominance Precedence Matrix
$$\text{SecurityBreach (Rank 5)} \succ \text{Freeze (Rank 4)} \succ \text{Refusal (Rank 3)} \succ \text{LedgerCorruption (Rank 2)} \succ \text{Pass (Rank 1)}$$

### Architectural Execution Rules
- **Same-Plane Lattice Joins:** Dual faults participating in a single evaluation plane resolve via the join-semilattice supremum ($x \sqcup y = \text{lub}\{x, y\}$). Dual oversized ($S > 4096\text{ B}$) and overtime ($\tau > 11,990\ \mu\text{s}$) conditions within `evaluate_frame_bounds()` resolve to **Freeze** ($\text{Rank 4} > \text{Rank 2}$).
- **Staged Short-Circuit Semantics:** Upstream architectural gates terminate processing *before* downstream predicates are reached. Ingress spatial rejection ($S > 4096\text{ B}$) terminates as `LedgerCorruption` (POSIX 30) before downstream $C_\text{ops}$ evaluation.
- **Identifier Separation:** Raw numeric POSIX values (40, 10, 32, 30, 0) are status identifiers, NOT dominance ranks. Poset order governs precedence.

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
# 1. Native Rust Substrate Verification
cargo test --lib

# 2. Cross-Language Lifecycle & Temporal Conformance Harness
npm test
