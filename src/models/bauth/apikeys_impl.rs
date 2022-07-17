use diesel::dsl::exists;
use diesel::*;
use uuid::Uuid;

use super::{ApiKey, ApiKeyDTO};
use crate::apierrors::ApiError;
use crate::models::schema::apikeys::dsl::{
    apikeys as dsl_apikeys, berta, customer_id, host_uuid, key,
};
use crate::models::{BaseCrud, DtoBase, ExtCrud};
use crate::ConnType;

impl ApiKey {
    pub fn get_by_key_and_owner(
        conn: &mut ConnType,
        cid: &Uuid,
        hkey: &str,
    ) -> Result<Self, ApiError> {
        Ok(dsl_apikeys
            .filter(customer_id.eq(cid).and(key.eq(hkey)))
            .first(conn)?)
    }

    pub fn get_by_key(conn: &mut ConnType, hkey: &str) -> Result<Self, ApiError> {
        Ok(dsl_apikeys.filter(key.eq(hkey)).first(conn)?)
    }

    pub fn get_by_key_berta(
        conn: &mut ConnType,
        hkey: &str,
        cberta: &str,
    ) -> Result<Self, ApiError> {
        Ok(dsl_apikeys
            .filter(key.eq(hkey).and(berta.eq(cberta)))
            .first(conn)?)
    }

    pub fn exists_by_owner_and_host(
        conn: &mut ConnType,
        cid: &Uuid,
        huuid: &str,
    ) -> Result<bool, ApiError> {
        Ok(select(exists(
            dsl_apikeys.filter(customer_id.eq(cid).and(host_uuid.eq(huuid))),
        ))
        .get_result(conn)?)
    }

    pub fn exists_by_owner_and_key(
        conn: &mut ConnType,
        cid: &Uuid,
        ckey: &str,
    ) -> Result<bool, ApiError> {
        Ok(select(exists(
            dsl_apikeys.filter(customer_id.eq(cid).and(key.eq(ckey))),
        ))
        .get_result(conn)?)
    }

    pub fn get_hosts_by_owner(
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

        Ok(res.into_iter().flatten().collect())
    }
}

impl<'a> BaseCrud<'a> for ApiKey {
    type RetType = ApiKey;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = i64;

    type UuidType = &'a Uuid;

    fn get(
        conn: &mut ConnType,
        uuid: Self::UuidType,
        size: i64,
        page: i64,
    ) -> Result<Self::VecRetType, ApiError> {
        Ok(dsl_apikeys
            .filter(customer_id.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(berta.asc())
            .load(conn)?)
    }

    fn get_specific(
        conn: &mut ConnType,
        target_id: Self::TargetType,
    ) -> Result<Self::RetType, ApiError> {
        Ok(dsl_apikeys.find(target_id).first(conn)?)
    }
}

impl<'a> ExtCrud<'a> for ApiKey {
    type UuidType = &'a Uuid;

    fn count(conn: &mut ConnType, uuid: Self::UuidType, _size: i64) -> Result<i64, ApiError> {
        Ok(dsl_apikeys
            .filter(customer_id.eq(uuid))
            .count()
            .get_result(conn)?)
    }
}

impl<'a> DtoBase<'a> for ApiKey {
    type GetReturn = ApiKey;

    type InsertType = &'a ApiKeyDTO;

    type UpdateType = Self::InsertType;

    type TargetType = &'a str;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_apikeys).values(value).execute(conn)?)
    }

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(insert_into(dsl_apikeys).values(value).get_result(conn)?)
    }

    fn update(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<usize, ApiError> {
        Ok(update(dsl_apikeys.filter(key.eq(target_id)))
            .set(value)
            .execute(conn)?)
    }

    fn update_and_get(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(update(dsl_apikeys.filter(key.eq(target_id)))
            .set(value)
            .get_result(conn)?)
    }

    fn delete(conn: &mut ConnType, target_id: Self::TargetType) -> Result<usize, ApiError> {
        Ok(delete(dsl_apikeys.filter(key.eq(target_id))).execute(conn)?)
    }
}
