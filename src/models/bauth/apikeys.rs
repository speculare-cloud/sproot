use crate::errors::AppError;
use crate::models::schema::apikeys;
use crate::models::schema::apikeys::dsl::{apikeys as dsl_apikeys, customer_id, host_uuid, key};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "apikeys"]
pub struct ApiKey {
    pub id: i32,
    pub key: String,
    pub host_uuid: String,
    pub customer_id: Uuid,
    pub berta: String,
}

impl ApiKey {
    /// Return a potential ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `hkey` - The apiKey, will be used for lookup
    pub fn get_entry(conn: &ConnType, hkey: &str) -> Result<Self, AppError> {
        Ok(dsl_apikeys.filter(key.eq(hkey)).first(conn)?)
    }

    /// Check if the entry exists for that pair of customer ID and host_uuid
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    /// * `uuid` - The host_uuid
    pub fn entry_exists(conn: &ConnType, cid: &Uuid, huuid: &str) -> Result<bool, AppError> {
        let res: Option<Self> = dsl_apikeys
            .filter(customer_id.eq(cid).and(host_uuid.eq(huuid)))
            .first(conn)
            .optional()?;

        Ok(res.is_some())
    }
}
