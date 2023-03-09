use std::{convert::Infallible, sync::Arc};

use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::sync::Mutex;
use warp::{hyper::StatusCode, Reply};

use crate::errors::db::DbError;

pub async fn list(
    db_session: Arc<Mutex<DatabaseConnection>>,
    _uid: (),
) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of users
    let db = db_session.lock().await.to_owned();
    let users: Vec<user::Model> = user::Entity::find().all(&db).await.unwrap();

    Ok(warp::reply::json(&users))
}

pub async fn get_by_id(
    id: i32,
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let user = user::Entity::find_by_id(id).one(&db).await;

    if user.is_err() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    let user = user.unwrap();

    if user.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::NOT_FOUND.as_str()),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    }

    Ok(warp::reply::json(&user.unwrap()).into_response())
}

pub async fn get_by_username(
    username: String,
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(&db)
        .await;

    if user.is_err() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    let user = user.unwrap();

    if user.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::NOT_FOUND.as_str()),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    }

    Ok(warp::reply::json(&user.unwrap()).into_response())
}

pub async fn create(
    user: user::ActiveModel,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<StatusCode, Infallible> {
    let db = db_session.lock().await.to_owned();
    log::debug!("create_user: {:?}", user);

    if check_user_by_username(db_session.clone(), &user.username.clone().unwrap())
        .await
        .is_err()
    {
        return Ok(StatusCode::BAD_REQUEST);
    }

    match user.insert(&db).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// pub async fn update(
//     id: String,
//     _uid: (),
//     user: user::Model,
//     db_session: Arc<Mutex<DatabaseConnection>>,
// ) -> Result<impl warp::Reply, Infallible> {
//     let db = db_session.lock().await.to_owned();
//     log::debug!("update_user: id={}, user={:?}", id, user);

//     if check_user_by_id(db_session.clone(), user.id).await.is_ok() {
//         return Ok(StatusCode::NOT_FOUND);
//     }

//     Ok(StatusCode::OK)
// }

// pub async fn delete(
//     id: i32,
//     _uid: (),
//     db_session: Arc<Mutex<DatabaseConnection>>,
// ) -> Result<impl warp::Reply, Infallible> {
//     let db = db_session.lock().await.to_owned();
//     log::debug!("delete_user: id={}", id);

//     if check_user_by_id(db_session.clone(), id).await.is_ok() {
//         return Ok(StatusCode::NOT_FOUND);
//     }

//     let result = false;

//     match result {
//         Ok(_) => Ok(StatusCode::NO_CONTENT),
//         Err(_) => Ok(StatusCode::SERVICE_UNAVAILABLE),
//     }
// }

pub async fn check_user_by_id(
    db_session: Arc<Mutex<DatabaseConnection>>,
    id: i32,
) -> Result<(), DbError> {
    let db = db_session.lock().await.to_owned();
    let db = db_session.lock().await.to_owned();
    let user = user::Entity::find_by_id(id).one(&db).await;

    if user.is_err() {
        return Err(DbError::InternalError);
    }

    let user = user.unwrap();

    if user.is_some() {
        return Err(DbError::AlreadyExists);
    }

    Ok(())
}

pub async fn check_user_by_username(
    db_session: Arc<Mutex<DatabaseConnection>>,
    username: &str,
) -> Result<(), DbError> {
    let db = db_session.lock().await.to_owned();
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(&db)
        .await;

    if user.is_err() {
        return Err(DbError::InternalError);
    }

    let user = user.unwrap();

    if user.is_some() {
        return Err(DbError::AlreadyExists);
    }

    Ok(())
}

// pub async fn get_id_by_token(
//     db_session: Arc<Mutex<DatabaseConnection>>,
//     token: &str,
// ) -> Result<i32, DbError> {
//     let db = db_session.lock().await.to_owned();
//
// }
