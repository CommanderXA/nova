use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::{handlers, models::role::Role, requests::post::create::PostCreateRequest};

use super::{with_auth, with_session};

pub fn posts(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create(session.clone())
        .or(list(session.clone()))
        .or(list_feed(session.clone()))
        .or(get(session.clone()))
        .or(update(session.clone()))
        .or(delete(session.clone()))
        .or(like(session.clone()))
}

pub fn list(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts")
        .and(warp::get())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::post::list)
}

pub fn list_feed(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("feed")
        .and(warp::get())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::post::list_feed)
}

pub fn create(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts")
        .and(warp::post())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and(json_body())
        .and_then(handlers::post::create)
}

pub fn get(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts" / i32)
        .and(warp::get())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::post::get_by_id)
}

pub fn like(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts" / i32 / "like")
        .and(warp::post())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::post::like)
}

pub fn update(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts" / i32)
        .and(warp::patch())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and(json_body())
        .and_then(handlers::post::update)
}

pub fn delete(
    session: Arc<Mutex<DatabaseConnection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("posts" / i32)
        .and(warp::delete())
        .and(with_auth(session.clone(), Role::User))
        .and(with_session(session))
        .and_then(handlers::post::delete)
}

fn json_body() -> impl Filter<Extract = (PostCreateRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
