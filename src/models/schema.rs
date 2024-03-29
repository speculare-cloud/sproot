// bSERVER models

table! {
    cpustats (id) {
        id -> Int8,
        interrupts -> Int8,
        ctx_switches -> Int8,
        soft_interrupts -> Int8,
        processes -> Int8,
        procs_running -> Int8,
        procs_blocked -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    cputimes (id) {
        id -> Int8,
        cuser -> Int8,
        nice -> Int8,
        system -> Int8,
        idle -> Int8,
        iowait -> Int8,
        irq -> Int8,
        softirq -> Int8,
        steal -> Int8,
        guest -> Int8,
        guest_nice -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    disks (id) {
        id -> Int8,
        disk_name -> Varchar,
        mount_point -> Varchar,
        total_space -> Int8,
        avail_space -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    hosts (uuid) {
        system -> Varchar,
        os_version -> Varchar,
        hostname -> Varchar,
        uptime -> Int8,
        sync_interval -> Int8,
        uuid -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ionets (id) {
        id -> Int8,
        interface -> Varchar,
        rx_bytes -> Int8,
        rx_packets -> Int8,
        rx_errs -> Int8,
        rx_drop -> Int8,
        tx_bytes -> Int8,
        tx_packets -> Int8,
        tx_errs -> Int8,
        tx_drop -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    ioblocks (id) {
        id -> Int8,
        device_name -> Varchar,
        read_count -> Int8,
        read_bytes -> Int8,
        write_count -> Int8,
        write_bytes -> Int8,
        busy_time -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    loadavg (id) {
        id -> Int8,
        one -> Float8,
        five -> Float8,
        fifteen -> Float8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    memory (id) {
        id -> Int8,
        total -> Int8,
        free -> Int8,
        used -> Int8,
        shared -> Int8,
        buffers -> Int8,
        cached -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    swap (id) {
        id -> Int8,
        total -> Int8,
        free -> Int8,
        used -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    cpustats, cputimes, disks, hosts, ionets, ioblocks, loadavg, memory, swap,
);

// !bSERVER models
// bALERTS models

table! {
    alerts (id) {
        id -> Int8,
        active -> Bool,
        _name -> Varchar,
        _table -> Varchar,
        lookup -> Text,
        timing -> Int4,
        warn -> Text,
        crit -> Text,
        info -> Nullable<Text>,
        host_uuid -> Varchar,
        cid -> Uuid,
        hostname -> Varchar,
        where_clause -> Nullable<Text>,
    }
}

table! {
    incidents (id) {
        id -> Int4,
        result -> Text,
        started_at -> Timestamp,
        updated_at -> Timestamp,
        resolved_at -> Nullable<Timestamp>,
        host_uuid -> Varchar,
        hostname -> Varchar,
        status -> Int4,
        severity -> Int4,
        alerts_id -> Int8,
        cid -> Uuid,
    }
}

allow_tables_to_appear_in_same_query!(incidents, alerts);

// !bALERTS models
// bAUTH models

table! {
    apikeys (id) {
        id -> Int8,
        key -> Text,
        host_uuid -> Nullable<Text>,
        customer_id -> Uuid,
        berta -> Text,
    }
}

table! {
    customers (id) {
        id -> Uuid,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(apikeys, customers);

// !bAUTH models
