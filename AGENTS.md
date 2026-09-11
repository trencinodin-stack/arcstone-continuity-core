AGENTS.md

This file is an operating map for autonomous agents working in this repository.
It is not a specification and does not establish Arcstone architectural authority.

Scope

This repository is the Path A public/open reference surface described in
README.md. It is not the complete Arcstone architecture.

Read README.md before modifying behavior.

Use:

src/ for executable Rust behavior.

tests/ for checked cross-language conformance behavior.

.github/workflows/substrate-ci.yml for the current verification path.

docs/ for broader architectural context and provenance.

Do not treat mechanisms described only in docs/ or cited publications as
implementation requirements for this repository.

Preserve semantic boundaries

Do not assume that architectural staging, local predicate evaluation, and
same-plane dominance are interchangeable.

Do not infer dominance from raw POSIX/status numeric identifiers.

Do not assume differences between Rust behavior and TypeScript conformance
behavior are defects to be reconciled. Check the governing README, source,
and tests before changing them.

The repository is FROZEN (ACTIVE). Do not "complete" broader Arcstone
architecture or alter established semantics merely because additional
mechanisms are described elsewhere.

Verify

Run the current CI verification path:

cargo test --lib
npm install
npm run test:unit -- tests/validateLifecycleInvariants.ts
npm run test:unit -- tests/clockDriftLattice.test.ts
