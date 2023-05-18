export interface CpuStats {
    id: bigint;
    interrupts: bigint;
    ctx_switches: bigint;
    soft_interrupts: bigint;
    processes: bigint;
    procs_running: bigint;
    procs_blocked: bigint;
    host_uuid: string;
    created_at: string;
}
