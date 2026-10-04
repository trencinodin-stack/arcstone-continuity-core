# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, buffer overflow risk, poset rank evaluation flaw, or execution-membrane logic bypass affecting `arcstone-continuity-core`, please **do not open a public issue**.

Submit reports through either of the following private channels:

1. **GitHub Private Vulnerability Reporting (Recommended):**
   * Go to the **Security** tab of this repository (`trencinodin-stack/arcstone-continuity-core`).
   * Click **Report a vulnerability**.
   * Fill out the form to submit a private report directly to maintainers.

2. **Email:**
   * Contact us at **security@arcstoneos.com**.

---

## What to Include

Please include, where applicable:
* A concise description of the vulnerability or predicate logic defect.
* Minimal reproduction steps or unit test (`cargo test`) demonstrating the issue.
* The affected crate version, release tag (e.g., `v1.3.1-exec`), or git commit hash (`A-77-DELTA-SHIELD-LOCKED`).
* Whether the issue involves:
  * Memory ceiling or boundary validation failure ($S_{max} > 4096\text{ B}$ buffer handling);
  * Temporal override or microsecond yield evaluation bypass ($\tau_{override} > 11,990\,\mu\text{s}$);
  * Poset dominance rank resolution or same-plane homomorphic join logic (`lifecycle.rs::evaluate_frame_bounds`);
  * POSIX status code identifier mapping mismatch per **ARC-ERR-2026-001**;
  * Ingress short-circuiting vs. local lattice evaluation ambiguity;
  * Violation of `#![no_std]` or `#![deny(unsafe_code)]` safety guarantees.

---

## Upstream Core Immutability & Errata Protocol

`arcstone-continuity-core` serves as the upstream public reference execution substrate operating under a strict Read-Only Locked Protocol Policy ($C_{ops} = 0$).

* **No Unilateral Parameter Mutation:** Reporting a defect does not authorize altering baseline thresholds (`MAX_BUFFER_BYTES`, `TAU_OVERRIDE_US`), POSIX status identifiers, or lattice dominance hierarchy without a formal update to **ARC-ERR-2026-001** (Master Canonical Errata, DOI `10.5281/zenodo.23069559`).
* **Preservation of Staged Semantics:** Fixes must preserve the separation between upstream architectural ingress short-circuiting (e.g., $S > 4096\text{ B}$ resolving to `POSIX 30`) and downstream state-vector predicate evaluation.

---

## Scope & Bounded Claims

`arcstone-continuity-core` is the Path A open-core reference implementation. Its security guarantees are limited strictly to deterministic $O(1)$ predicate evaluation ($\Pi(S)$) and `#![no_std]` memory/temporal safety.

The repository explicitly does **not** claim:
* Production hardware or eBPF microkernel enforcement (hosted in `Arcstone OS`);
* Dynamic capability issuance or external claim consumption (handled downstream in `arcstone-mcp-sidecar`);
* Non-deterministic candidate proposal containment (handled downstream in `arcstone-adaptive-producer-lab`);
* Complete system-wide end-to-end authorization security beyond local predicate evaluation.

---

## Response Timeline

We will acknowledge receipt within **48 hours** and work with you on a coordinated resolution and disclosure timeline.
