use super::{Customers, CustomersDTO};

use crate::errors::AppError;
use crate::models::schema::customers::dsl::{customers as dsl_customers, email, id};
use crate::ConnType;

use diesel::*;
use uuid::Uuid;

impl Customers {
    /// Return the customers with the corresponding email
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `mail` - The email address of the customer
    pub fn get(conn: &mut ConnType, mail: &str) -> Result<Self, AppError> {
        Ok(dsl_customers.filter(email.eq(mail)).first(conn)?)
    }

    /// Return a bool which tell us if a customer exists
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The UUID of the customer
    pub fn exists(conn: &mut ConnType, cid: &Uuid) -> Result<bool, AppError> {
        let res: Option<Self> = dsl_customers.filter(id.eq(cid)).first(conn).optional()?;

        Ok(res.is_some())
    }
}

impl<'a> CustomersDTO<'a> {
    /// Create a new customer and return the number of row affected (1)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn insert(&self, conn: &mut ConnType) -> Result<usize, AppError> {
        Ok(insert_into(dsl_customers).values(self).execute(conn)?)
    }

    /// Return the newly created customer
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn ginsert(&self, conn: &mut ConnType) -> Result<Customers, AppError> {
        Ok(insert_into(dsl_customers).values(self).get_result(conn)?)
    }
}
