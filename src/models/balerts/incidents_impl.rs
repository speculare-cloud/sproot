use super::{Incidents, IncidentsDTO, IncidentsDTOUpdate};

use crate::apierrors::ApiError;
use crate::models::schema::incidents::dsl::{
    alerts_id, host_uuid, id, incidents as dsl_incidents, status, updated_at,
};
use crate::ConnType;

use diesel::*;

impl Incidents {
    /// Return a Vector of Incidents for a specific host
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get incidents of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_list_host(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        // Depending on if the uuid is present or not
        Ok(dsl_incidents
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(updated_at.desc())
            .load(conn)?)
    }

    /// Determine if an incident for that specific alert exists and is currently active.
    /// If one is found, return it, otherwise return a Err(NotFound).
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the alert related to the incident
    pub fn find_active(conn: &mut ConnType, target_id: &str) -> Result<Self, ApiError> {
        Ok(dsl_incidents
            .filter(alerts_id.eq(target_id).and(status.eq(0)))
            .first(conn)?)
    }

    /// Remove an Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the incident to delete
    pub fn delete(conn: &mut ConnType, target_id: i32) -> Result<usize, ApiError> {
        Ok(delete(dsl_incidents.filter(id.eq(target_id))).execute(conn)?)
    }

    /// Return the numbers of Incidents within a size limit
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get incidents of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &mut ConnType, uuid: &str, size: i64) -> Result<i64, ApiError> {
        Ok(dsl_incidents
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .count()
            .get_result(conn)?)
    }
}

impl IncidentsDTO {
    /// Insert new Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The Incidents struct containing the new incident information
    pub fn insert(&self, conn: &mut ConnType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_incidents).values(self).execute(conn)?)
    }

    /// Insert a new Incident inside the database and return the inserted row
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The Incident struct containing the new incident information
    pub fn ginsert(&self, conn: &mut ConnType) -> Result<Incidents, ApiError> {
        Ok(insert_into(dsl_incidents).values(self).get_result(conn)?)
    }
}

impl IncidentsDTOUpdate {
    /// Update an Incidents inside the database and return the updated Struct
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The HttpIncidents struct containing the updated incident information
    /// * `target_id` - The id of the incident to update
    pub fn gupdate(&self, conn: &mut ConnType, target_id: i32) -> Result<Incidents, ApiError> {
        Ok(update(dsl_incidents.filter(id.eq(target_id)))
            .set(self)
            .get_result(conn)?)
    }
}

impl From<Incidents> for IncidentsDTO {
    fn from(incident: Incidents) -> IncidentsDTO {
        IncidentsDTO {
            result: incident.result,
            started_at: incident.started_at,
            updated_at: incident.updated_at,
            resolved_at: incident.resolved_at,
            host_uuid: incident.host_uuid,
            hostname: incident.hostname,
            status: incident.status,
            severity: incident.severity,
            alerts_id: incident.alerts_id,
            alerts_name: incident.alerts_name,
            alerts_table: incident.alerts_table,
            alerts_lookup: incident.alerts_lookup,
            alerts_warn: incident.alerts_warn,
            alerts_crit: incident.alerts_crit,
            alerts_info: incident.alerts_info,
            alerts_where_clause: incident.alerts_where_clause,
        }
    }
}

impl From<Incidents> for IncidentsDTOUpdate {
    fn from(incident: Incidents) -> IncidentsDTOUpdate {
        IncidentsDTOUpdate {
            result: Some(incident.result),
            started_at: Some(incident.started_at),
            updated_at: Some(incident.updated_at),
            resolved_at: incident.resolved_at,
            host_uuid: Some(incident.host_uuid),
            hostname: Some(incident.hostname),
            status: Some(incident.status),
            severity: Some(incident.severity),
            alerts_id: Some(incident.alerts_id),
            alerts_name: Some(incident.alerts_name),
            alerts_table: Some(incident.alerts_table),
            alerts_lookup: Some(incident.alerts_lookup),
            alerts_warn: Some(incident.alerts_warn),
            alerts_crit: Some(incident.alerts_crit),
            alerts_info: incident.alerts_info,
            alerts_where_clause: incident.alerts_where_clause,
        }
    }
}
