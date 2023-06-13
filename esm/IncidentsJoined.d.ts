import type { Alerts } from "./Alerts";
export interface IncidentsJoined {
    id: number;
    result: string;
    started_at: string;
    updated_at: string;
    resolved_at: string | null;
    host_uuid: string;
    hostname: string;
    status: number;
    severity: number;
    alerts_id: bigint;
    cid: string;
    alert: Alerts | null;
}
