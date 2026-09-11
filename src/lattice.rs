// src/lattice.rs
// Bound strictly to Target Master Hash: A-77-DELTA-SHIELD-LOCKED

use crate::PosixSignal;

/// Enforces the Order-Theoretic 5-Tier Dominance Lattice Laws:
/// SECURITY_BREACH (5) > FREEZE (4) > REFUSAL (3) > LEDGER_CORRUPTION (2) > PASS (1)
pub struct DominanceLattice;

impl DominanceLattice {
    /// Resolves conflicting operational markers to determine absolute state prioritization.
    pub fn resolve_precedence(sig_a: PosixSignal, sig_b: PosixSignal) -> PosixSignal {
        let rank_a = Self::get_rank(sig_a);
        let rank_b = Self::get_rank(sig_b);

        if rank_a >= rank_b { sig_a } else { sig_b }
    }

    fn get_rank(signal: PosixSignal) -> u8 {
        match signal {
            PosixSignal::SecurityBreach => 5, // Dominates all other signals
            PosixSignal::Freeze => 4,         // Clock/Temporal stasis holds
            PosixSignal::Refusal => 3,        // Queue saturation floors
            PosixSignal::LedgerCorruption => 2,// Expansion bounds structural rejects
            PosixSignal::Pass => 1,           // Baseline nominal clearance
        }
    }
}
