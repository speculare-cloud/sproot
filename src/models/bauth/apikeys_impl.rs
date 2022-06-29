use super::{ApiKey, ApiKeyDTO};

use crate::apierrors::ApiError;
use crate::models::schema::apikeys::dsl::{
    apikeys as dsl_apikeys, berta, customer_id, host_uuid, id, key,
};
use crate::ConnType;

use diesel::*;
use uuid::Uuid;

impl ApiKey {
    /// Return a Vec of ApiKeys
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    pub fn get_keys(conn: &mut ConnType, cid: &Uuid) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_apikeys.filter(customer_id.eq(cid)).get_results(conn)?)
    }

    /// Return a Vec of ApiKeys
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    /// * `hkey` - The apiKey, will be used for lookup
    pub fn get_key_owned(conn: &mut ConnType, cid: &Uuid, hkey: &str) -> Result<Self, ApiError> {
        Ok(dsl_apikeys
            .filter(customer_id.eq(cid).and(key.eq(hkey)))
            .first(conn)?)
    }

    /// Return a potential ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `hkey` - The apiKey, will be used for lookup
    pub fn get_entry(conn: &mut ConnType, hkey: &str) -> Result<Self, ApiError> {
        Ok(dsl_apikeys.filter(key.eq(hkey)).first(conn)?)
    }

    /// Return a potential ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `hkey` - The apiKey, will be used for lookup
    /// * `cberta` - The supposed berta where the key is registered
    pub fn get_entry_berta(
        conn: &mut ConnType,
        hkey: &str,
        cberta: &str,
    ) -> Result<Self, ApiError> {
        Ok(dsl_apikeys
            .filter(key.eq(hkey).and(berta.eq(cberta)))
            .first(conn)?)
    }

    /// Check if the entry exists for that pair of customer ID and host_uuid
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    /// * `uuid` - The host_uuid
    pub fn entry_exists(conn: &mut ConnType, cid: &Uuid, huuid: &str) -> Result<bool, ApiError> {
        let res: Option<Self> = dsl_apikeys
            .filter(customer_id.eq(cid).and(host_uuid.eq(huuid)))
            .first(conn)
            .optional()?;

        Ok(res.is_some())
    }

    /// Check if the cid (user) owns the key
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    /// * `ckey` - The API key
    pub fn own_key(conn: &mut ConnType, cid: &Uuid, ckey: &str) -> Result<bool, ApiError> {
        let res: Option<Self> = dsl_apikeys
            .filter(customer_id.eq(cid).and(key.eq(ckey)))
            .first(conn)
            .optional()?;

        Ok(res.is_some())
    }

    /// Get a list of Some(host_uuid) owned by the customer
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    pub fn get_hosts_by_owned(
        conn: &mut ConnType,
        cid: &Uuid,
        size: i64,
        page: i64,
    ) -> Result<Vec<String>, ApiError> {
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
    pub fn delete_key(conn: &mut ConnType, target_key: &str) -> Result<usize, ApiError> {
        Ok(delete(dsl_apikeys.filter(key.eq(target_key))).execute(conn)?)
    }
}

impl ApiKeyDTO {
    /// Create a new key and return the number of row affected (1)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn insert(&self, conn: &mut ConnType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_apikeys).values(self).execute(conn)?)
    }

    /// Return the newly created ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn ginsert(&self, conn: &mut ConnType) -> Result<ApiKey, ApiError> {
        Ok(insert_into(dsl_apikeys).values(self).get_result(conn)?)
    }

    /// Update a specific ApiKey using the target_id and return the number of row affected (1)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The targeted ApiKey to update
    pub fn update(&self, conn: &mut ConnType, target_id: i64) -> Result<usize, ApiError> {
        Ok(update(dsl_apikeys.filter(id.eq(target_id)))
            .set(self)
            .execute(conn)?)
    }

    /// Return the updated ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The targeted ApiKey to update
    pub fn gupdate(&self, conn: &mut ConnType, target_id: i64) -> Result<ApiKey, ApiError> {
        Ok(update(dsl_apikeys.filter(id.eq(target_id)))
            .set(self)
            .get_result(conn)?)
    }
}
