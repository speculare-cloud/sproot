use diesel::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::schema::apikeys;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize, TS)]
#[diesel(table_name = apikeys)]
#[ts(export)]
pub struct ApiKey {
	#[ts(type = "number")]
    pub id: i64,
    pub key: String,
    pub host_uuid: Option<String>,
    pub customer_id: Uuid,
    pub berta: String,
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(Insertable, AsChangeset, Deserialize, Serialize, Debug, Default)]
#[diesel(table_name = apikeys)]
pub struct ApiKeyDTO {
    pub key: Option<String>,
    pub host_uuid: Option<String>,
    pub customer_id: Option<Uuid>,
    pub berta: Option<String>,
}
