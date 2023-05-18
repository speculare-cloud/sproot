export interface IoNet {
    id: bigint;
    interface: string;
    rx_bytes: bigint;
    rx_packets: bigint;
    rx_errs: bigint;
    rx_drop: bigint;
    tx_bytes: bigint;
    tx_packets: bigint;
    tx_errs: bigint;
    tx_drop: bigint;
    host_uuid: string;
    created_at: string;
}
