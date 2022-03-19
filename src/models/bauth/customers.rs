use crate::errors::AppError;
use crate::models::schema::customers;
use crate::models::schema::customers::dsl::{customers as dsl_customers, email, id};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "customers"]
pub struct Customers {
    pub id: Uuid,
    pub email: String,
}

impl Customers {
    /// Return the customers with the corresponding email
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `mail` - The email address of the customer
    pub fn get(conn: &ConnType, mail: &str) -> Result<Self, AppError> {
        Ok(dsl_customers.filter(email.eq(mail)).first(conn)?)
    }

    /// Return a bool which tell us if a customer exists
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The UUID of the customer
    pub fn exists(conn: &ConnType, cid: &Uuid) -> Result<bool, AppError> {
        let res: Option<Self> = dsl_customers.filter(id.eq(cid)).first(conn).optional()?;

        Ok(res.is_some())
    }
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "customers"]
pub struct CustomersDTO<'a> {
    pub email: &'a str,
}

impl<'a> CustomersDTO<'a> {
    /// Create a new customer and return the number of row affected (1)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn insert(&self, conn: &ConnType) -> Result<usize, AppError> {
        Ok(insert_into(dsl_customers).values(self).execute(conn)?)
    }

    /// Return the newly created customer
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn ginsert(&self, conn: &ConnType) -> Result<Customers, AppError> {
        Ok(insert_into(dsl_customers).values(self).get_result(conn)?)
    }
}
