use std::sync::Arc;

use entity::user::Model as User;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use warp::Filter;

use crate::{handlers, models::role::Role};

use super::{with_auth, with_session};

pub fn users(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    users_list(session.clone())
        .or(users_get_by_id(session.clone()))
        .or(users_by_username(session.clone()))
        // .or(users_update(session.clone()))
        // .or(users_delete(session))
}

/// GET /users
pub fn users_list(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_session(session.clone()))
        .and(with_auth(session, Role::User))
        .and_then(handlers::users::list)
}

/// GET /users/:username
pub fn users_by_username(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::users::get_by_username)
}

/// GET /users/:uuid
pub fn users_get_by_id(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / i32)
        .and(warp::get())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::users::get_by_id)
}

/// POST /users with JSON body
// pub fn users_create(
//     session: Arc<Mutex<DatabaseConnection>>,
// ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
//     warp::path!("users")
//         .and(warp::post())
//         .and(json_body())
//         .and(with_session(session))
//         .and_then(handlers::users::create)
// }

/// PUT /users/:id with JSON body
// pub fn users_update(
//     session: Arc<Mutex<DatabaseConnection>>,
// ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
//     warp::path!("users" / String)
//         .and(warp::put())
//         .and(with_auth(session.clone(), Role::User))
//         .and(json_body())
//         .and(with_session(session))
//         .and_then(handlers::users::update)
// }

/// DELETE /users/:id
// pub fn users_delete(
//     session: Arc<Mutex<DatabaseConnection>>,
// ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
//     warp::path!("users" / i32)
//         // It is important to put the auth check _after_ the path filters.
//         // If we put the auth check before, the request `PUT /users/invalid-string`
//         // would try this filter and reject because the authorization header doesn't match,
//         // rather because the param is wrong for that other path.
//         .and(warp::delete())
//         .and(with_auth(session.clone(), Role::User))
//         .and(with_session(session))
//         .and_then(handlers::users::delete)
// }

pub fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
