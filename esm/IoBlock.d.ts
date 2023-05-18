export interface IoBlock {
    id: bigint;
    device_name: string;
    read_count: bigint;
    read_bytes: bigint;
    write_count: bigint;
    write_bytes: bigint;
    busy_time: bigint;
    host_uuid: string;
    created_at: string;
}
