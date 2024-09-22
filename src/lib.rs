#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

pub mod apierrors;
pub mod models;

use std::fs::File;
use std::io::BufReader;
use std::{ffi::OsStr, path::Path};

use actix_session::config::{CookieContentSecurity, PersistentSession};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    ServerConfig,
};

use crate::apierrors::ApiError;

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Assert that a field is defined or return an error
#[macro_export]
macro_rules! field_isset {
    ($value:expr, $name:literal) => {
        match $value {
            Some(x) => Ok(x),
            None => Err(ApiError::ServerError(Some(format!(
                "isset: optional field {} is not defined but is needed",
                $name
            )))),
        }
    };
}

/// Evaluate an Enum into the value it hold
#[macro_export]
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

/// "Unwrap" result and if error return pretty error + fatal exit
#[macro_export]
macro_rules! unwrapf {
    ($value:expr) => {
        match $value {
            Ok(x) => x,
            Err(err) => {
                error!("result failed: {}", err);
                std::process::exit(1);
            }
        }
    };
}

/// Simple function returning the name of the binary
///
/// Use to filter the logs.
pub fn prog() -> Option<String> {
    std::env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

/// Return the ServerConfig needed for Actix to be bind on HTTPS
///
/// Use key and cert for the path to find the files.
pub fn get_ssl_builder(key: &str, cert: &str) -> Result<ServerConfig, ApiError> {
    let mut reader = BufReader::new(File::open(key)?);

    // Try to read PKCS8 private keys first
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map(|v| v.unwrap().clone_key())
        .collect::<Vec<_>>();

    let key = if !keys.is_empty() {
        PrivateKeyDer::Pkcs8(keys[0].clone_key())
    } else {
        // If no PKCS8 keys are found, try to read RSA private keys
        let mut reader = BufReader::new(File::open(key)?);
        let keys = rustls_pemfile::rsa_private_keys(&mut reader)
            .map(|v| v.unwrap().clone_key())
            .collect::<Vec<_>>();

        if !keys.is_empty() {
            PrivateKeyDer::Pkcs1(keys[0].clone_key())
        } else {
            return Err(ApiError::ExplicitError("No private keys found".to_owned()));
        }
    };

    let cert_file = &mut BufReader::new(File::open(cert)?);
    // Create a Vec of certificate by extracting all cert from cert_file
    let cert_chain = rustls_pemfile::certs(cert_file)
        .map(|v| CertificateDer::from_slice(&v.unwrap().clone()).into_owned())
        .collect();

    // Return the ServerConfig to be used
    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?)
}

/// Get the SessionMiddleware with configured Cookie settings
///
/// The hard-coded default session-length is 12 weeks (+/- 3 months).
/// The content of the cookie (session) is not crypted and is simply
/// signed. So do not store any confidential information in it.
pub fn get_session_middleware(
    secret: &[u8],
    cookie_name: String,
    cookie_domain: Option<String>,
) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        actix_web::cookie::Key::from(secret),
    )
    .session_lifecycle(
        PersistentSession::default().session_ttl(actix_web::cookie::time::Duration::weeks(12)),
    )
    .cookie_domain(cookie_domain)
    .cookie_name(cookie_name)
    .cookie_content_security(CookieContentSecurity::Signed)
    .build()
}
