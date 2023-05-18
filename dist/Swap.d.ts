export interface Swap {
    id: bigint;
    total: bigint;
    free: bigint;
    used: bigint;
    host_uuid: string;
    created_at: string;
}
