use super::{Alerts, AlertsConfig, HostTargeted};

use crate::errors::AppError;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid, id};
use crate::ConnType;

use diesel::*;
use std::path::PathBuf;
use walkdir::WalkDir;

impl Alerts {
    pub fn generate_id_from(uuid: &str, name: &str) -> String {
        sha1_smol::Sha1::from([uuid.as_bytes(), name.as_bytes()].concat()).hexdigest()
    }

    /// Build from a AlertsConfig, host_uuid, hostname and an id.
    pub fn build_from_config(
        config: AlertsConfig,
        uuid: String,
        hostname: String,
        alert_id: String,
    ) -> Alerts {
        Alerts {
            id: alert_id,
            name: config.name,
            table: config.table,
            lookup: config.lookup,
            timing: config.timing,
            warn: config.warn,
            crit: config.crit,
            info: config.info,
            host_uuid: uuid,
            hostname,
            where_clause: config.where_clause,
        }
    }

    /// Get a list of alerts
    pub fn get_list(conn: &mut ConnType) -> Result<Vec<Self>, AppError> {
        Ok(dsl_alerts.order_by(_name.asc()).load(conn)?)
    }

    /// Get a list of alerts for the specific host
    pub fn get_list_host(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(_name.asc())
            .load(conn)?)
    }

    /// Get a specific alert
    pub fn get(conn: &mut ConnType, target_id: &str) -> Result<Self, AppError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }

    /// Insert one or multiple alerts
    pub fn insert(conn: &mut ConnType, alerts: &[Alerts]) -> Result<usize, AppError> {
        Ok(insert_into(dsl_alerts).values(alerts).execute(conn)?)
    }

    /// Insert and get the result of the alerts inserted
    pub fn ginsert(conn: &mut ConnType, alerts: &[Alerts]) -> Result<Vec<Self>, AppError> {
        Ok(insert_into(dsl_alerts).values(alerts).get_results(conn)?)
    }

    /// Delete one alert
    pub fn delete(conn: &mut ConnType, target_id: &str) -> Result<usize, AppError> {
        Ok(delete(dsl_alerts.filter(id.eq(target_id))).execute(conn)?)
    }

    /// Delete all alert (shouldn't be used)
    pub fn delete_all(conn: &mut ConnType) -> Result<usize, AppError> {
        Ok(delete(dsl_alerts).execute(conn)?)
    }
}

impl AlertsConfig {
    /// Construct AlertsConfig Vec from the path of configs's folder & sub
    #[allow(clippy::result_unit_err)]
    pub fn from_configs_path(path: &str) -> Result<Vec<AlertsConfig>, AppError> {
        let mut alerts: Vec<AlertsConfig> = Vec::new();

        if std::fs::metadata(path).is_err() {
            return Err(AppError::new("error: alerts_path: not found".to_owned()));
        }

        for entry in WalkDir::new(&path).min_depth(1).max_depth(2) {
            // Detect if the WalkDir failed to read the folder (permissions/...)
            let entry = entry?;

            // Skip if folder
            if entry.path().is_dir() {
                continue;
            }

            // Get the parent folder name and determine which hosts is targeted
            let parent_entry = entry
                .path()
                .parent()
                .ok_or_else(|| AppError::new("error: .path().parent() returned None".to_owned()))?;

            let host_targeted = if parent_entry == PathBuf::from(&path) {
                HostTargeted::ALL
            } else {
                let parent_name = parent_entry.file_name().ok_or_else(|| {
                    AppError::new("error: parent_entry.file_name() returned None".to_owned())
                })?;

                HostTargeted::SPECIFIC(
                    parent_name
                        .to_str()
                        .ok_or_else(|| {
                            AppError::new("error: parent_name.to_str() returned None".to_owned())
                        })?
                        .to_owned(),
                )
            };

            trace!(
                "Alerts {:?}; HostTargeted[{:?}]",
                entry.path().file_name(),
                host_targeted,
            );

            // Read and store the content of the config into a string
            let mut content = std::fs::read_to_string(entry.path())?;
            // Deserialize the string's config into the struct of AlertsConfig
            let mut alert_config = simd_json::from_str::<AlertsConfig>(&mut content)?;
            alert_config.host_targeted = Some(host_targeted);

            // Add the AlertsConfig into the Vec
            alerts.push(alert_config);
        }

        Ok(alerts)
    }
}
