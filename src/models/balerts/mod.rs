mod alerts;
mod alerts_impl;
mod alerts_querying;
pub use alerts::*;
pub use alerts_impl::*;
pub use alerts_querying::*;

mod incidents;
mod incidents_impl;
pub use incidents::*;
pub use incidents_impl::*;
use once_cell::sync::Lazy;
use regex::Regex;

pub mod qtype;

static INTERVAL_RGX: Lazy<Regex> = Lazy::new(|| {
    match Regex::new(r"(\d+)([a-zA-Z' '])|([m,h,d,minutes,hours,days,minute,hour,day])") {
        Ok(reg) => reg,
        Err(e) => {
            error!("Cannot build the Regex to validate INTERVAL: {}", e);
            std::process::exit(1);
        }
    }
});

/// Constant list of disallowed statement in the SQL query to avoid somthg bad
pub const DISALLOWED_STATEMENT: &[&str] = &[
    "DELETE",
    "UPDATE",
    "INSERT",
    //"CREATE", => conflict with created_at, TODO FIX LATER
    "ALTER",
    "DROP",
    "TRUNCATE",
    "GRANT",
    "REVOKE",
    "BEGIN",
    "COMMIT",
    "SAVEPOINT",
    "ROLLBACK",
];

/// Represente the type of the Query an alert ask for
#[derive(Debug, PartialEq, Clone)]
pub enum QueryType {
    Pct,
    Abs,
}
