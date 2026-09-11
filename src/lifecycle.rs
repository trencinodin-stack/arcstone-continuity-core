use crate::{PosixSignal, MAX_BUFFER_BYTES, TAU_OVERRIDE_US, Micros};
use crate::lattice::DominanceLattice;

/// Evaluates execution frame invariants using lattice-homomorphic precedence join.
///
/// Takes a direct byte slice `payload` to ensure physical length binding,
/// and a strongly-typed `Micros` duration to eliminate unit mismatch.
pub fn evaluate_frame_bounds(payload: &[u8], elapsed: Micros) -> PosixSignal {
    let mut signal = PosixSignal::Pass;

    if payload.len() > MAX_BUFFER_BYTES {
        signal = DominanceLattice::resolve_precedence(signal, PosixSignal::LedgerCorruption);
    }

    if elapsed.0 > TAU_OVERRIDE_US {
        signal = DominanceLattice::resolve_precedence(signal, PosixSignal::Freeze);
    }

    signal
}
