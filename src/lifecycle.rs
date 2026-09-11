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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_fault_lattice_homomorphism() {
        let small_payload = [0u8; 100];
        let large_payload = [0u8; 5000];

        // 1. Pass / Pass -> Pass
        assert_eq!(
            evaluate_frame_bounds(&small_payload, Micros(5000)),
            PosixSignal::Pass
        );

        // 2. Oversized / Normal Time -> LedgerCorruption
        assert_eq!(
            evaluate_frame_bounds(&large_payload, Micros(5000)),
            PosixSignal::LedgerCorruption
        );

        // 3. Normal Size / Overtime -> Freeze
        assert_eq!(
            evaluate_frame_bounds(&small_payload, Micros(15000)),
            PosixSignal::Freeze
        );

        // 4. Dual Fault (Oversized AND Overtime) -> Freeze dominates LedgerCorruption
        assert_eq!(
            evaluate_frame_bounds(&large_payload, Micros(15000)),
            PosixSignal::Freeze
        );
    }
}
