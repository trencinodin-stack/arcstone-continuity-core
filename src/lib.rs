// src/lib.rs
// Bound strictly to Target Master Hash: A-77-DELTA-SHIELD-LOCKED
// System Baseline Release Tag: v1.3.0-exec
// Copyright (c) 2026 Arcstone Adaptive Science Systems, Inc. All rights reserved.

#![no_std]
#![deny(unsafe_code)]

pub mod lifecycle;
pub mod lattice;

pub const MAX_BUFFER_BYTES: usize = 4096; // INV-MEM-01 Memory Cap
pub const TAU_OVERRIDE_US: u64 = 11990;     // INV-TIME-02 Temporal Cap (11.99ms)

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PosixSignal {
    Pass = 0x00,              // POSIX 0: Authorize Core Execution Space Ingestion
    Freeze = 0x0A,            // POSIX 10: Clock Drift Override / Stasis Latch
    LedgerCorruption = 0x1E,  // POSIX 30: Buffer Expansion / Size Breach (>4KB)
    Refusal = 0x20,           // POSIX 32: Queue Saturation Ceiling (rho >= 0.95)
    SecurityBreach = 0x28,    // POSIX 40: Critical Invariant Violation / Lockout
}
