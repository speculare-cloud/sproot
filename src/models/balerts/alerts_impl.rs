use diesel::dsl::exists;
use diesel::sql_types::{BigInt, Text};
use diesel::*;
use uuid::Uuid;

use super::{Alerts, AlertsDTO, AlertsDTOUpdate, HttpAlertsCount, QueryType};
use crate::apierrors::ApiError;
use crate::models::balerts::INTERVAL_RGX;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid};
use crate::models::schema::alerts::{cid, id};
use crate::models::{BaseCrud, DtoBase, ExtCrud, DISALLOWED_STATEMENT};
use crate::ConnType;

pub trait AlertsQuery {
    fn construct_query(&self) -> Result<(String, QueryType), ApiError>;
}

trait AlertsDTOTrait {
    fn g_lookup(&self) -> &String;
    fn g_where_clause(&self) -> &Option<String>;
    fn g_table(&self) -> &String;
    fn g_name(&self) -> &String;
    fn g_host_uuid(&self) -> &String;
}

impl AlertsDTOTrait for Alerts {
    #[inline]
    fn g_lookup(&self) -> &String {
        &self.lookup
    }

    #[inline]
    fn g_where_clause(&self) -> &Option<String> {
        &self.where_clause
    }

    #[inline]
    fn g_table(&self) -> &String {
        &self.table
    }

    #[inline]
    fn g_name(&self) -> &String {
        &self.name
    }

    #[inline]
    fn g_host_uuid(&self) -> &String {
        &self.host_uuid
    }
}

impl AlertsDTOTrait for AlertsDTO {
    #[inline]
    fn g_lookup(&self) -> &String {
        &self.lookup
    }

    #[inline]
    fn g_where_clause(&self) -> &Option<String> {
        &self.where_clause
    }

    #[inline]
    fn g_table(&self) -> &String {
        &self.table
    }

    #[inline]
    fn g_name(&self) -> &String {
        &self.name
    }

    #[inline]
    fn g_host_uuid(&self) -> &String {
        &self.host_uuid
    }
}

impl Alerts {
    /// Get all the Alerts (no filter, nothing, just get all)
    /// - conn: the Database connection
    pub fn get_all(conn: &mut ConnType) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_alerts.load(conn)?)
    }

    /// Is the alert owned by the user
    /// - conn: the Database connection
    /// - cid: the user's UUID
    /// - aid: the id of the alert you want to check
    pub fn exists_by_owner_and_id(
        conn: &mut ConnType,
        ccid: &Uuid,
        aid: i64,
    ) -> Result<bool, ApiError> {
        Ok(select(exists(dsl_alerts.filter(cid.eq(ccid).and(id.eq(aid))))).get_result(conn)?)
    }
}

impl<'a> BaseCrud<'a> for Alerts {
    type RetType = Alerts;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = i64;

    type UuidType = &'a str;

    /// Get all the Alerts defined for a specific host
    /// - conn: the Database connection
    /// - uuid: the targeted's host_uuid
    /// - size: how many elements to return
    /// - page: pagination :shrug:
    fn get(
        conn: &mut ConnType,
        uuid: Self::UuidType,
        size: i64,
        page: i64,
    ) -> Result<Self::VecRetType, ApiError> {
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(_name.asc())
            .load(conn)?)
    }

    /// Get a specific Alert depending on the target_id
    /// - conn: the Database connection
    /// - target_id: the targeted alert's id
    fn get_specific(
        conn: &mut ConnType,
        target_id: Self::TargetType,
    ) -> Result<Self::RetType, ApiError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }
}

impl<'a> ExtCrud<'a> for Alerts {
    type UuidType = &'a str;
    type RetType = HttpAlertsCount;

    fn count(
        conn: &mut ConnType,
        uuid: Self::UuidType,
        size: i64,
    ) -> Result<Self::RetType, ApiError> {
        Ok(sql_query(
            "
			SELECT
				COUNT(*) FILTER (WHERE active = true) as active,
				COUNT(*) FILTER (WHERE active = false) as inactive,
				COUNT(*) as total
			FROM alerts
			WHERE host_uuid=$1
			LIMIT $2
		",
        )
        .bind::<Text, _>(uuid)
        .bind::<BigInt, _>(size)
        .get_result(conn)?)
    }
}

impl<'a> DtoBase<'a> for Alerts {
    type GetReturn = Vec<Alerts>;

    type InsertType = &'a [AlertsDTO];

    type UpdateType = &'a AlertsDTOUpdate;

    type TargetType = i64;

    type UpdateReturnType = Alerts;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError> {
        Ok(insert_into(dsl_alerts).values(value).execute(conn)?)
    }

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError> {
        Ok(insert_into(dsl_alerts).values(value).get_results(conn)?)
    }

    fn update(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<usize, ApiError> {
        Ok(update(dsl_alerts.filter(id.eq(target_id)))
            .set(value)
            .execute(conn)?)
    }

    fn update_and_get(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<Self::UpdateReturnType, ApiError> {
        Ok(update(dsl_alerts.filter(id.eq(target_id)))
            .set(value)
            .get_result(conn)?)
    }

    fn delete(conn: &mut ConnType, target_id: Self::TargetType) -> Result<usize, ApiError> {
        Ok(delete(dsl_alerts.find(target_id)).execute(conn)?)
    }
}

impl<T> AlertsQuery for T
where
    T: AlertsDTOTrait,
{
    fn construct_query(&self) -> Result<(String, QueryType), ApiError> {
        // Split the lookup String from the alert for analysis
        let lookup_parts: Vec<&str> = self.g_lookup().split(' ').collect();

        // Assert that we have enough parameters
        if lookup_parts.len() < 5 {
            return Err(ApiError::InvalidRequestError(Some(String::from("query: the lookup query is invalid, define as follow: [aggr] [mode] [timeframe] of [table] {over} {table}"))));
        }

        // Determine the mode of the query it's for now, either Pct or Abs
        let req_mode = match lookup_parts[1] {
            "pct" => QueryType::Pct,
            "abs" => QueryType::Abs,
            _ => {
                return Err(ApiError::InvalidRequestError(Some(format!(
                    "query: mode {} is invalid. Valid are: pct, abs.",
                    lookup_parts[1]
                ))));
            }
        };

        // If we're in mode Pct, we need more than 5 parts
        if req_mode == QueryType::Pct && lookup_parts.len() != 7 {
            return Err(ApiError::InvalidRequestError(Some(String::from(
                "query: lookup defined as mode pct but missing values, check usage.",
            ))));
        }

        // The type of the query this is pretty much the aggregation function Postgres is going to use
        let req_aggr = lookup_parts[0];
        // Assert that req_type is correct (avg, sum, min, max, count)
        if !["avg", "sum", "min", "max", "count"].contains(&req_aggr) {
            return Err(ApiError::InvalidRequestError(Some(String::from(
                "query: aggr is invalid. Valid are: avg, sum, min, max, count.",
            ))));
        }

        // Get the timing of the query, that is the interval range
        let req_time = lookup_parts[2];
        // Assert that req_time is correctly formatted (Regex?)
        if !INTERVAL_RGX.is_match(req_time) {
            return Err(ApiError::InvalidRequestError(Some(String::from(
                "query: req_time is not correctly formatted (doesn't pass regex).",
            ))));
        }

        // This is the columns we ask for in the first place, this value is mandatory
        let req_one = lookup_parts[4];

        // Construct the SELECT part of the query
        let mut pg_select = String::new();
        let select_cols = req_one.split(',');
        for col in select_cols {
            // We're casting everything to float8 to handle pretty much any type we need
            pg_select.push_str(&format!("{}({})::float8 + ", req_aggr, col));
        }
        // Remove the last " + "
        pg_select.drain(pg_select.len() - 3..pg_select.len());
        // Based on the mode, we might need to do some different things
        match req_mode {
            // For pct we need to define numerator and divisor.
            QueryType::Pct => {
                // req_two only exists if req_mode == Pct
                let req_two = lookup_parts[6];

                pg_select.push_str(" as numerator, ");
                let select_cols = req_two.split(',');
                for col in select_cols {
                    pg_select.push_str(&format!("{}({})::float8 + ", req_aggr, col));
                }
                pg_select.drain(pg_select.len() - 3..pg_select.len());
                pg_select.push_str(" as divisor");
            }
            // For abs we just need to define the addition of all columns as value
            QueryType::Abs => {
                pg_select.push_str(" as value");
            }
        }

        // Optional where clause
        // Allow us to add a WHERE condition to the query if needed
        let mut pg_where = String::new();
        if let Some(where_clause) = &self.g_where_clause() {
            pg_where.push_str(" AND ");
            pg_where.push_str(where_clause);
        }

        // Base of the query, we plug every pieces together here
        let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", req_time, pg_select, self.g_table(), pg_where);

        trace!("Query[{:?}] is {}", req_mode, &query);

        // Assert that we don't have any malicious statement in the query
        // by changing it to uppercase and checking against our list of banned statement.
        let tmp_query = query.to_uppercase();
        for statement in DISALLOWED_STATEMENT {
            if tmp_query.contains(statement) {
                return Err(ApiError::InvalidRequestError(Some(format!(
                    "Alert {} for host_uuid {:.6} contains disallowed statement \"{}\"",
                    self.g_name(),
                    self.g_host_uuid(),
                    statement
                ))));
            }
        }

        Ok((query, req_mode))
    }
}
