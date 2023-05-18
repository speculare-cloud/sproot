export interface Disk {
    id: bigint;
    disk_name: string;
    mount_point: string;
    total_space: bigint;
    avail_space: bigint;
    host_uuid: string;
    created_at: string;
}
