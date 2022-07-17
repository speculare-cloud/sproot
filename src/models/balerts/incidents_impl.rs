use diesel::*;

use super::{Incidents, IncidentsDTO, IncidentsDTOUpdate};
use crate::apierrors::ApiError;
use crate::models::schema::incidents::dsl::{
    alerts_id, host_uuid, id, incidents as dsl_incidents, status, updated_at,
};
use crate::models::{BaseCrud, DtoBase, ExtCrud};
use crate::ConnType;

impl Incidents {
    pub fn find_active(conn: &mut ConnType, aid: &str) -> Result<Self, ApiError> {
        Ok(dsl_incidents
            .filter(alerts_id.eq(aid).and(status.eq(0)))
            .first(conn)?)
    }
}

impl<'a> BaseCrud<'a> for Incidents {
    type RetType = Incidents;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = i32;

    type UuidType = &'a str;

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
    ) -> Result<Self::GetReturn, ApiError> {
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
        }
    }
}
