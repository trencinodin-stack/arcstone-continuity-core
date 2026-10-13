pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/comparators.circom";

template DAEVerifier() {
    // --- Private Inputs ---
    signal input execution_time_us;
    signal input heap_bytes;
    signal input poset_code;

    // --- Public Inputs ---
    signal input expected_poset_code;

    // --- Output ---
    signal output is_admissible;

    // --- Invariants ---
    // WCET <= 11990 microseconds (11.99 ms)
    component timeCheck = LessEqThan(32);
    timeCheck.in[0] <== execution_time_us;
    timeCheck.in[1] <== 11990;

    // Heap Bytes == 0
    component heapCheck = IsZero();
    heapCheck.in <== heap_bytes;

    // Poset Code Match
    component posetCheck = IsEqual();
    posetCheck.in[0] <== poset_code;
    posetCheck.in[1] <== expected_poset_code;

    // Combine All Constraints
    signal intermediate;
    intermediate <== timeCheck.out * heapCheck.out;
    is_admissible <== intermediate * posetCheck.out;

    // Enforce Admissibility
    is_admissible === 1;
}

component main {public [expected_poset_code]} = DAEVerifier();
