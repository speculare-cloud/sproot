use super::Alerts;

use crate::apierrors::ApiError;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid, id};
use crate::ConnType;

use diesel::*;

impl Alerts {
    pub fn generate_id_from(uuid: &str, name: &str) -> String {
        sha1_smol::Sha1::from([uuid.as_bytes(), name.as_bytes()].concat()).hexdigest()
    }

    /// Get a list of alerts
    pub fn get_list(conn: &mut ConnType) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_alerts.order_by(_name.asc()).load(conn)?)
    }

    /// Get a list of alerts for the specific host
    pub fn get_list_host(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(_name.asc())
            .load(conn)?)
    }

    /// Get a specific alert
    pub fn get(conn: &mut ConnType, target_id: &str) -> Result<Self, ApiError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }

    /// Insert one or multiple alerts
    pub fn insert(conn: &mut ConnType, alerts: &[Alerts]) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_alerts).values(alerts).execute(conn)?)
    }

    /// Insert and get the result of the alerts inserted
    pub fn ginsert(conn: &mut ConnType, alerts: &[Alerts]) -> Result<Vec<Self>, ApiError> {
        Ok(insert_into(dsl_alerts).values(alerts).get_results(conn)?)
    }

    /// Delete one alert
    pub fn delete(conn: &mut ConnType, target_id: &str) -> Result<usize, ApiError> {
        Ok(delete(dsl_alerts.filter(id.eq(target_id))).execute(conn)?)
    }

    /// Delete all alert (shouldn't be used)
    pub fn delete_all(conn: &mut ConnType) -> Result<usize, ApiError> {
        Ok(delete(dsl_alerts).execute(conn)?)
    }
}
