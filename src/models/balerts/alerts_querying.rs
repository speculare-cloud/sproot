use diesel::{
    sql_types::{Float8, Timestamp},
    QueryableByName,
};

/// Struct to hold the return from the sql_query for percentage query
#[derive(QueryableByName, Debug)]
pub struct PctDTORaw {
    #[diesel(sql_type = Float8)]
    pub numerator: f64,
    #[diesel(sql_type = Float8)]
    pub divisor: f64,
    #[diesel(sql_type = Timestamp)]
    pub time: chrono::NaiveDateTime,
}

/// Struct to hold the return from the sql_query for absolute query
#[derive(QueryableByName, Debug)]
pub struct AbsDTORaw {
    #[diesel(sql_type = Float8)]
    pub value: f64,
    #[diesel(sql_type = Timestamp)]
    pub time: chrono::NaiveDateTime,
}
