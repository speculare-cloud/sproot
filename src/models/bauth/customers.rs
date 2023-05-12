use diesel::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::schema::customers;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize, TS)]
#[diesel(table_name = customers)]
#[ts(export)]
pub struct Customers {
    pub id: Uuid,
    pub email: String,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = customers)]
pub struct CustomersDTO<'a> {
    pub email: &'a str,
}
