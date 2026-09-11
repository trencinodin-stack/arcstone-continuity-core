#![no_std]

pub const MAX_BUFFER_BYTES: usize = 4096;
pub const TAU_OVERRIDE_US: u64 = 11_990;

/// Type-safe microsecond wrapper to prevent ambient unit drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Micros(pub u64);

/// POSIX-mapped 5-tier dominance status lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PosixSignal {
    Pass = 0x00,
    LedgerCorruption = 0x1E,
    Refusal = 0x20,
    Freeze = 0x0A,
    SecurityBreach = 0x28,
}
