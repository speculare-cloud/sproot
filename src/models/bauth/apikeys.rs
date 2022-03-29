use crate::errors::AppError;
use crate::models::schema::apikeys;
use crate::models::schema::apikeys::dsl::{
    apikeys as dsl_apikeys, customer_id, host_uuid, id, key,
};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "apikeys"]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub host_uuid: Option<String>,
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

    /// Get a list of Some(host_uuid) owned by the customer
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    pub fn get_host_by_owned(
        conn: &ConnType,
        cid: &Uuid,
        size: i64,
        page: i64,
    ) -> Result<Vec<String>, AppError> {
        let res = dsl_apikeys
            .select(host_uuid)
            .filter(customer_id.eq(cid).and(host_uuid.is_not_null()))
            .limit(size)
            .offset(page * size)
            .order_by(host_uuid.asc())
            .get_results::<Option<String>>(conn)?;
        // Remove None from the Vec
        Ok(res.into_iter().flatten().collect())
    }

    /// Delete the specified key returning the number of row affected. (1 if found)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_key` - The API key to delete
    pub fn delete_key(conn: &ConnType, target_key: &str) -> Result<usize, AppError> {
        Ok(delete(dsl_apikeys.filter(key.eq(target_key))).execute(conn)?)
    }
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(AsChangeset, Deserialize, Serialize, Debug, Default)]
#[table_name = "apikeys"]
pub struct ApiKeyDTOUpdate {
    pub key: Option<String>,
    pub host_uuid: Option<String>,
    pub customer_id: Option<Uuid>,
    pub berta: Option<String>,
}

impl ApiKeyDTOUpdate {
    pub fn update(&self, conn: &ConnType, target_id: i64) -> Result<usize, AppError> {
        Ok(update(dsl_apikeys.filter(id.eq(target_id)))
            .set(self)
            .execute(conn)?)
    }

    pub fn gupdate(&self, conn: &ConnType, target_id: i64) -> Result<ApiKey, AppError> {
        Ok(update(dsl_apikeys.filter(id.eq(target_id)))
            .set(self)
            .get_result(conn)?)
    }
}
