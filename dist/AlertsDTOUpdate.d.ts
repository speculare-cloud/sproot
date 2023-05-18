export interface AlertsDTOUpdate {
    active: boolean | null;
    name: string | null;
    table: string | null;
    lookup: string | null;
    timing: number | null;
    warn: string | null;
    crit: string | null;
    info: string | null;
    where_clause: string | null;
}
