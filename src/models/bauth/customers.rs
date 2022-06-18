use crate::models::schema::customers;

use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = customers)]
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
