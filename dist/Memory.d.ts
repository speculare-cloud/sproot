export interface Memory {
    id: bigint;
    total: bigint;
    free: bigint;
    used: bigint;
    shared: bigint;
    buffers: bigint;
    cached: bigint;
    host_uuid: string;
    created_at: string;
}
