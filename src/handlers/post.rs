use std::{convert::Infallible, sync::Arc};

use entity::{post, prelude::Post};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tokio::sync::Mutex;
use warp::{hyper::StatusCode, Reply};

use crate::requests::post::create::PostCreateRequest;

pub async fn list(
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db_session.lock().await.to_owned();
    let posts: Vec<post::Model> = post::Entity::find().all(&db).await.unwrap();

    Ok(warp::reply::json(&posts))
}

pub async fn get_by_id(
    id: i32,
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    let post = post.unwrap();

    if post.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::NOT_FOUND.as_str()),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    }

    Ok(warp::reply::json(&post.unwrap()).into_response())
}

pub async fn create(
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
    req: PostCreateRequest,
) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let post = post::ActiveModel {
        user_id: Set(req.user_id),
        related_to_post: Set(req.related_to_post),
        text: Set(req.text),
        ..Default::default()
    };

    match post.insert(&db).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    id: i32,
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
    req: PostCreateRequest,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();

    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    let post = post.unwrap();
    if post.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::NOT_FOUND.as_str()),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    }

    let mut post: post::ActiveModel = post.unwrap().into();
    post.text = Set(req.text);

    match post.update(&db).await {
        Ok(post) => {
            Ok(warp::reply::with_status(warp::reply::json(&post), StatusCode::OK).into_response())
        }
        Err(_e) => Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()),
    }
}

pub async fn delete(
    id: i32,
    _uid: (),
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();

    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::INTERNAL_SERVER_ERROR.as_str()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    let post = post.unwrap();

    if post.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&StatusCode::NOT_FOUND.as_str()),
            StatusCode::NOT_FOUND,
        )
        .into_response());
    }

    match Post::delete_by_id(id).exec(&db).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}
