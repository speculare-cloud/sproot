use diesel::*;

use super::Alerts;
use crate::apierrors::ApiError;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid};
use crate::models::{BaseCrud, DtoBase, ExtCrud};
use crate::ConnType;

impl Alerts {
    /// Get all the Alerts (no filter, nothing, just get all)
    /// - conn: the Database connection
    pub fn get_all(conn: &mut ConnType) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_alerts.load(conn)?)
    }
}

impl<'a> BaseCrud<'a> for Alerts {
    type RetType = Alerts;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = &'a str;

    type UuidType = &'a str;

    /// Get all the Alerts defined for a specific host
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
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(_name.asc())
            .load(conn)?)
    }

    /// Get a specific Alert depending on the target_id
    /// - conn: the Database connection
    /// - target_id: the targeted alert's id
    fn get_specific(conn: &mut ConnType, target_id: &str) -> Result<Self::RetType, ApiError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }
}

impl<'a> ExtCrud<'a> for Alerts {
    type UuidType = &'a str;

    fn count(conn: &mut ConnType, uuid: Self::UuidType, size: i64) -> Result<i64, ApiError> {
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .count()
            .get_result(conn)?)
    }
}

impl<'a> DtoBase<'a> for Alerts {
    type GetReturn = Vec<Alerts>;

    type InsertType = &'a [Alerts];

    type UpdateType = Self::InsertType;

    type TargetType = &'a str;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_alerts).values(value).execute(conn)?)
    }

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(insert_into(dsl_alerts).values(value).get_results(conn)?)
    }

    fn update(
        _conn: &mut ConnType,
        _target_id: Self::TargetType,
        _value: Self::UpdateType,
    ) -> Result<usize, ApiError> {
        todo!()
    }

    fn update_and_get(
        _conn: &mut ConnType,
        _target_id: Self::TargetType,
        _value: Self::UpdateType,
    ) -> Result<Self::GetReturn, ApiError> {
        todo!()
    }

    fn delete(conn: &mut ConnType, target_id: Self::TargetType) -> Result<usize, ApiError> {
        Ok(delete(dsl_alerts.find(target_id)).execute(conn)?)
    }
}
