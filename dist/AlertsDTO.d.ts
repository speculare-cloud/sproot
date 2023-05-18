export interface AlertsDTO {
    active: boolean | null;
    name: string;
    table: string;
    lookup: string;
    timing: number;
    warn: string;
    crit: string;
    info: string | null;
    host_uuid: string;
    cid: string;
    hostname: string;
    where_clause: string | null;
}
