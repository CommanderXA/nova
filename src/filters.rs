use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use warp::{header::headers_cloned, http::HeaderValue, hyper::HeaderMap, Filter, Rejection};

use crate::models::role::Role;

use self::auth::authorize;

pub mod auth;
pub mod users;
pub mod posts;

pub fn with_session(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<DatabaseConnection>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || session.clone())
}

pub fn with_auth(
    session: Arc<Mutex<DatabaseConnection>>,
    role: Role,
) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and(with_session(session))
        .and_then(authorize)
}
