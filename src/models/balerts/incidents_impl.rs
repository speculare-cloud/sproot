use diesel::*;
use uuid::Uuid;

use super::{Alerts, Incidents, IncidentsDTO, IncidentsDTOUpdate, IncidentsJoined};
use crate::apierrors::ApiError;
use crate::models::schema::{
    alerts::{self, dsl::id as alid},
    incidents::{
        self,
        dsl::{alerts_id, cid, host_uuid, id, incidents as dsl_incidents, status, updated_at},
    },
};
use crate::models::{BaseCrud, DtoBase, ExtCrud};
use crate::ConnType;

impl Incidents {
    /// Get the active incident for the specific alert (if any)
    /// - conn: the Database connection
    /// - aid: the targeted alert's id
    ///
    /// In theory there should at most be one active incidents
    /// per alert per host. If there's more than one it's not handled.
    pub fn find_active(conn: &mut ConnType, aid: i64) -> Result<Self, ApiError> {
        Ok(dsl_incidents
            .filter(alerts_id.eq(aid).and(status.eq(0)))
            .first(conn)?)
    }

    /// Get the incidents of that particular Uuid (user)
    /// - conn: the Database connection
    /// - uuid: the user UUID we want the incidents of
    /// - size: how many elements to return
    /// - page: pagination :shrug:
    pub fn get_owned(
        conn: &mut ConnType,
        uuid: &Uuid,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_incidents
            .filter(cid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(updated_at.desc())
            .load(conn)?)
    }

    /// Get the incidents of that particular Uuid (user) linked with the alerts
    ///
    /// Note: if the alerts does not exists anymore, it won't return the incident field neither
    /// TOOD - Take care of this case
    /// - conn: the Database connection
    /// - uuid: the user UUID we want the incidents of
    /// - size: how many elements to return
    /// - page: pagination :shrug:
    pub fn get_own_joined(
        conn: &mut ConnType,
        uuid: &Uuid,
        size: i64,
        page: i64,
    ) -> Result<Vec<IncidentsJoined>, ApiError> {
        Ok(incidents::table
            .filter(cid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(updated_at.desc())
            .inner_join(alerts::table.on(alerts_id.eq(alid)))
            .load::<(Self, Alerts)>(conn)
            .map(|x| x.into_iter().map(IncidentsJoined::from))?
            .collect::<Vec<_>>())
    }
}

impl<'a> BaseCrud<'a> for Incidents {
    type RetType = Incidents;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = i32;

    type UuidType = &'a str;

    /// Get all the Incidents defined for a specific host
    /// - conn: the Database connection
    /// - uuid: the targeted's host_uuid
    /// - size: how many elements to return
    /// - page: pagination :shrug:
    fn get(
        conn: &mut ConnType,
        uuid: Self::UuidType,
        size: i64,
        page: i64,
    ) -> Result<Self::VecRetType, ApiError> {
        Ok(dsl_incidents
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(updated_at.desc())
            .load(conn)?)
    }

    /// Get a specific Incident depending on the target_id
    /// - conn: the Database connection
    /// - target_id: the targeted incident's id
    fn get_specific(
        conn: &mut ConnType,
        target_id: Self::TargetType,
    ) -> Result<Self::RetType, ApiError> {
        Ok(dsl_incidents.find(target_id).first(conn)?)
    }
}

impl<'a> ExtCrud<'a> for Incidents {
    type UuidType = &'a str;

    fn count(conn: &mut ConnType, uuid: Self::UuidType, size: i64) -> Result<i64, ApiError> {
        Ok(dsl_incidents
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .count()
            .get_result(conn)?)
    }
}

impl<'a> DtoBase<'a> for Incidents {
    type GetReturn = Incidents;

    type InsertType = &'a IncidentsDTO;

    type UpdateType = &'a IncidentsDTOUpdate;

    type TargetType = i32;

    type UpdateReturnType = Self::GetReturn;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_incidents).values(value).execute(conn)?)
    }

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(insert_into(dsl_incidents).values(value).get_result(conn)?)
    }

    fn update(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<usize, ApiError> {
        Ok(update(dsl_incidents.filter(id.eq(target_id)))
            .set(value)
            .execute(conn)?)
    }

    fn update_and_get(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<Self::UpdateReturnType, ApiError> {
        Ok(update(dsl_incidents.filter(id.eq(target_id)))
            .set(value)
            .get_result(conn)?)
    }

    fn delete(conn: &mut ConnType, target_id: Self::TargetType) -> Result<usize, ApiError> {
        Ok(delete(dsl_incidents.find(target_id)).execute(conn)?)
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
            cid: incident.cid,
        }
    }
}

impl From<Incidents> for IncidentsDTOUpdate {
    fn from(incident: Incidents) -> IncidentsDTOUpdate {
        IncidentsDTOUpdate {
            result: Some(incident.result),
            updated_at: Some(incident.updated_at),
            resolved_at: incident.resolved_at,
            status: Some(incident.status),
            severity: Some(incident.severity),
        }
    }
}
