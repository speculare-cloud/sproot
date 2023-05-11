use diesel::dsl::exists;
use diesel::*;
use uuid::Uuid;

use super::{Customers, CustomersDTO};
use crate::apierrors::ApiError;
use crate::models::schema::customers::dsl::{customers as dsl_customers, email, id};
use crate::models::DtoBase;
use crate::ConnType;

impl Customers {
    /// Get the user object by email address
    /// - conn: the Database connection
    /// - mail: the email address of the user
    pub fn get_specific(conn: &mut ConnType, mail: &str) -> Result<Customers, ApiError> {
        Ok(dsl_customers.filter(email.eq(mail)).first(conn)?)
    }

    /// Does the user exists?
    /// - conn: the Database connection
    /// - cid: the user's UUID
    pub fn exists(conn: &mut ConnType, cid: &Uuid) -> Result<bool, ApiError> {
        Ok(select(exists(dsl_customers.filter(id.eq(cid)))).get_result(conn)?)
    }
}

impl<'a> DtoBase<'a> for Customers {
    type GetReturn = Customers;

    type InsertType = &'a CustomersDTO<'a>;

    // useless :)
    type UpdateType = i8;

    type TargetType = &'a Uuid;

    type UpdateReturnType = Self::GetReturn;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_customers).values(value).execute(conn)?)
    }

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(insert_into(dsl_customers).values(value).get_result(conn)?)
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
        Ok(delete(dsl_customers.find(target_id)).execute(conn)?)
    }
}
