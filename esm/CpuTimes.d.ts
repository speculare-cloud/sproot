export interface CpuTimes {
    id: bigint;
    cuser: bigint;
    nice: bigint;
    system: bigint;
    idle: bigint;
    iowait: bigint;
    irq: bigint;
    softirq: bigint;
    steal: bigint;
    guest: bigint;
    guest_nice: bigint;
    host_uuid: string;
    created_at: string;
}
