# Client Integration & Deterministic Safety Contract

**Document Reference:** `ARC-SPEC-2026-CLI-01`  
**Status:** `RELEASE_CANDIDATE_BASELINE`  
**Temporal Anchor:** Sep 10, 2026, 12:25 PM  
**Target Integration Hash:** `A-77-DELTA-SHIELD-LOCKED`

---

## 1. Asynchronous Hold Buffer (TRQ) Saturation & Refusal Receipts (`POSIX 32`)

### 1.1 Architectural Rationale & The $C_{\text{ops}} = 0$ Invariant
The Arcstone Continuity Core operates under a non-negotiable zero operational drag invariant ($C_{\text{ops}} = 0$). The real-time execution fabric must never stall, block, or incur synchronous computational overhead due to downstream client state failures or congestion.

When the Asynchronous Reconciliation Queue (TRQ) allocation metric ($\rho$) reaches critical saturation ($\rho \ge 0.95$), the ingress proxy immediately short-circuits processing prior to heavy parsing to prevent CPU starvation and volatile memory fragmentation. The edge layer instantly emits a signed refusal receipt accompanied by a `POSIX 32` (`REFUSAL`) exit code.

```text
[Client Request] ---> [eBPF Edge Ingress Proxy]
                             |
                    (Is rho >= 0.95?)
                    /               \
                 YES                 NO
                 /                     \
    [POSIX 32 Refusal Receipt]    [Ingested into TRQ Buffer]
    (No Client Retries Allowed)   (Asynchronous Execution)
1.2 Mandatory Client-Side BehaviorClient applications interacting with the edge gateway are strictly prohibited from implementing unthrottled or infinite retry loops upon receiving a POSIX 32 signal. Rapid repetitive requests directly undermine the $C_{\text{ops}} = 0$ baseline by creating unnecessary packet triage overhead at the eBPF layer.Clients MUST adhere to the following execution sequence upon catching a POSIX 32 token:Immediate Circuit Break: Halt the active request pipeline for the affected capability domain.Hard Stop or Localized Evacuation: For non-essential telemetry, drop the payload immediately. For critical financial or operational asset deltas, serialize the payload locally to an offline-first state-based CRDT log chain.Controlled Exponential Backoff with Jitter: Requests may only resume after an extended backoff window, calculated with mandatory randomization to eliminate limit-cycle synchronization anomalies across downstream systems.2. Epoch Expiration & Nonce Lifetimes (POSIX 10 / 0x10B0)2.1 The Sub-12ms Temporal BoundaryTo eliminate time-manipulation vulnerabilities, replay vectors, and token reuse exploits across distributed enclaves, the Core Execution Membrane replaces wall-clock tracking with hardware-backed monotonic clocks paired with sequence epoch numbers.The system enforces a strict sub-12ms hardware temporal override ceiling:$$\tau_{\text{override}} \le 11.99\text{ms}$$Processing windows degrade predictably across fixed microarchitectural boundaries:Nominal Processing Window ($\tau < 11.00\text{ms}$): Emits POSIX 0 (PASS).Soft Yield Window ($11.00\text{ms} \le \tau < 11.99\text{ms}$): Preemptively serialized and evacuated out-of-band to the TRQ via POSIX 10 (FREEZE) with sub-status 0x10A0 (PWC Hold).Hard Control Ceiling ($\tau \ge 11.99\text{ms}$): Triggers a Non-Maskable Interrupt (NMI) that zeroizes volatile execution registers and locks the bare-metal circuit breaker with POSIX 40 (SECURITY_BREACH).2.2 Client-Side Timestamp SynchronizationEvery probabilistic state transition emitted by client models must be cryptographically bound to an ephemeral, hardware-backed monotonic nonce before submission to the Admissibility Control Plane (ACP). These nonces reside within Non-Cacheable Tightly-Coupled Memory (TCM) and possess a rigid Time-To-Live (TTL) of exactly 1 Epoch ($11.99\text{ms}$).Clients must maintain clock synchronization with the hardware lattice timeline using peer-to-peer vector clock tracking. If a client payload presents an unverified nonce or a timestamp skewed by more than 50 monotonic ticks relative to the local clock instance, the ingress membrane triggers an atomic bitset swap, collapsing the frame to POSIX 10 (Sub-Status 0x10B0) (Clock Drift Stasis).3. Toolchain & Proof Baseline Lock (asmrefine)3.1 Mathematical Soundness and Binary ParityThe core safety properties, invariant enforcement layers, and fail-closed dominance semantics of the arcstone-continuity-core are mathematically guaranteed through formal proofs authored in the Isabelle/HOL interactive theorem prover. These proofs establish functional correctness at the source-code level.To bridge the gap between abstract mathematical source specifications and low-level machine instructions executing on bare metal, the framework utilizes asmrefine (an automated binary translation validation pipeline). This pipeline formally proves that the compiled assembly instructions structurally conform to the validated invariants without introducing compiler tail-padding drift or unauthorized memory layout mutations.3.2 Invalidation of the Verification BaselineModifying compilation flags, introducing custom linker scripts, or altering optimization profiles (e.g., changing from -C opt-level=z to -O3) completely invalidates the asmrefine proof matrix:Variable-length footprint fluctuations introduce non-deterministic Worst-Case Execution Time (WCET) bounds.Altered instruction ordering can introduce subtle cache-line timing leaks or break the strict 64-byte packed memory alignment layout required by the ingestion gates.Consequently, the core toolchain configuration is locked. Any downstream deployment modified by compilation flags will automatically fail build-time static verification checks and refuse to generate a valid AAC-CERT cryptographic execution token.4. Reference Implementation: Compliant Client HandlerThe following TypeScript interface and client handler demonstrate compliant request framing, epoch synchronization tracking, and strict POSIX signal enforcement:TypeScript// docs/spec/client-integration.ts
// Compliance Baseline: ARC-SPEC-2026-CLI-01
// Target Integration Hash: A-77-DELTA-SHIELD-LOCKED

export enum PosixSignal {
    PASS = 0x0000,
    PWC_HOLD = 0x10A0,
    CLOCK_FREEZE = 0x10B0,
    LEDGER_CORRUPTION = 0x001E,
    REFUSAL = 0x0020,
    SECURITY_BREACH = 0x0028
}

export interface IIntegrationClientConfig {
    maxRetries: number;
    baseBackoffMs: number;
    maxBackoffMs: number;
    localTcmClockOffsetTicks: number;
}

export interface ICoreResponse {
    posixCode: number;
    subStatusMask: string;
    proofSignature: string | null;
    payload: string;
}

export class DeterministicSafetyGatewayClient {
    private config: IIntegrationClientConfig;
    private isCircuitBroken: boolean = false;

    constructor(config: IIntegrationClientConfig) {
        this.config = config;
    }

    /**
     * Executes an outbound payload delivery to the eBPF Core Ingress.
     * Guarantees adherence to the C_ops = 0 drag invariant.
     */
    public async submitCoreTransaction(
        actionId: string, 
        rawPayload: string, 
        currentEpochSequence: number
    ): Promise<ICoreResponse> {
        
        if (this.isCircuitBroken) {
            throw new Error(`CRITICAL_CLIENT_ABORT: Pipeline isolated due to active circuit breaker.`);
        }

        let retryCount = 0;

        while (retryCount < this.config.maxRetries) {
            try {
                const response = await this.executeNetworkTransmit(actionId, rawPayload, currentEpochSequence);

                switch (response.posixCode) {
                    case PosixSignal.PASS:
                        return response;

                    case PosixSignal.PWC_HOLD:
                    case PosixSignal.CLOCK_FREEZE:
                        // POSIX 10: Evacuated to out-of-band TRQ stasis. 
                        // Synchronous thread drops immediately. Local tracking maps to async listener.
                        this.logAsynchronousOffload(actionId, response.subStatusMask);
                        return response;

                    case PosixSignal.REFUSAL:
                        // POSIX 32: Queue Saturation (rho >= 0.95). Infinite loops strictly forbidden.
                        this.enforceCircuitBreaker(`Queue saturation detected. Ingress refused.`);
                        return response;

                    case PosixSignal.LEDGER_CORRUPTION:
                        // POSIX 30: Size breach or malformed structural JCS frame.
                        throw new Error(`TERMINAL_FRAME_ERROR: Ingress rejected for structural corruption.`);

                    case PosixSignal.SECURITY_BREACH:
                        // POSIX 40: Critical temporal breach or adversarial pattern trap.
                        this.enforceCircuitBreaker(`Security breach short-circuit encountered.`);
                        return response;

                    default:
                        this.enforceCircuitBreaker(`Unrecognized POSIX token sequence returned.`);
                        return response;
                }

            } catch (error) {
                retryCount++;
                if (retryCount >= this.config.maxRetries) {
                    this.enforceCircuitBreaker(`Maximum client retry bounds exhausted: ${error}`);
                    throw error;
                }
                
                // Implement required Exponential Backoff with Decorrelated Jitter
                await this.executeJitteredSleep(retryCount);
            }
        }

        throw new Error("UNREACHABLE_SAFETY_STATE");
    }

    private enforceCircuitBreaker(reason: string): void {
        this.isCircuitBroken = true;
        console.error(`[CIRCUIT_BREAK] Invariant protection activated. Reason: ${reason}`);
    }

    private async executeJitteredSleep(retryAttempt: number): Promise<void> {
        const calculateBackoff = Math.min(
            this.config.maxBackoffMs,
            this.config.baseBackoffMs * Math.pow(2, retryAttempt)
        );
        // Apply decorrelated random jitter to protect edge packet processors
        const jitteredDelay = Math.random() * calculateBackoff;
        return new Promise(resolve => setTimeout(resolve, jitteredDelay));
    }

    private async executeNetworkTransmit(
        actionId: string, 
        payload: string, 
        epoch: number
    ): Promise<ICoreResponse> {
        // Core execution network mapping implementation goes here
        return { posixCode: 0, subStatusMask: "0x0000", proofSignature: "0x...", payload: "ACK" };
    }

    private logAsynchronousOffload(actionId: string, mask: string): void {
        console.log(`[TRQ_OFFLOAD] Context serialized out-of-band for Action: ${actionId}, Mask: ${mask}`);
    }
}
