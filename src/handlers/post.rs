use std::{convert::Infallible, sync::Arc};

use entity::{follower, post, post_like, prelude::Post};
use migration::{Alias, DbErr, JoinType, Order};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
};
use tokio::sync::Mutex;
use warp::{hyper::StatusCode, Reply};

use crate::requests::post::create::PostCreateRequest;

pub async fn list(
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db_session.lock().await.to_owned();
    let posts: Vec<post::Model> = post::Entity::find().all(&db).await.unwrap();

    Ok(warp::reply::json(&posts))
}

pub async fn list_feed(
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    let db = db_session.lock().await.to_owned();

    let followings: Result<Vec<follower::Model>, DbErr> = follower::Entity::find()
        .filter(follower::Column::FollowerId.eq(_id_from_token))
        .all(&db)
        .await;

    if followings.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    let followings = followings
        .unwrap()
        .into_iter()
        .map(|user| user.user_id)
        .collect::<Vec<i32>>();

    let posts: Vec<post::Model> = post::Entity::find()
        .filter(post::Column::UserId.is_in(followings))
        .join_as(
            JoinType::LeftJoin,
            post::Relation::User.def(),
            Alias::new("user"),
        )
        .order_by(post::Column::CreatedAt, Order::Desc)
        .all(&db)
        .await
        .unwrap();

    Ok(warp::reply::json(&posts).into_response())
}

pub async fn get_by_id(
    id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    let post = post.unwrap();
    if post.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    Ok(warp::reply::json(&post.unwrap()).into_response())
}

pub async fn like(
    post_id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db_session.lock().await.to_owned();

    let post_like = post_like::Entity::find_by_id((_id_from_token, post_id))
        .one(&db)
        .await;

    if post_like.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let post = post::Entity::find_by_id(post_id).one(&db).await;
    if post.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let post = post.unwrap();
    if post.is_none() {
        return Ok(StatusCode::NOT_FOUND);
    }

    let mut post_model: post::ActiveModel = post.unwrap().into();
    match post_like.unwrap() {
        Some(item) => {
            let txn = db.begin().await.unwrap();
            item.delete(&txn).await.unwrap();

            post_model.likes = Set(post_model.likes.take().unwrap() - 1);
            post_model.save(&txn).await.unwrap();

            txn.commit().await.unwrap();

            Ok(StatusCode::OK)
        }
        None => {
            let txn = db.begin().await.unwrap();

            post_like::ActiveModel {
                post_id: Set(post_id),
                user_id: Set(_id_from_token),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .unwrap();

            post_model.likes = Set(post_model.likes.take().unwrap() + 1);
            post_model.save(&txn).await.unwrap();

            txn.commit().await.unwrap();

            Ok(StatusCode::CREATED)
        }
    }
}

pub async fn create(
    _id_from_token: i32,
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
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
    req: PostCreateRequest,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();

    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    let post = post.unwrap();
    if post.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let mut post: post::ActiveModel = post.unwrap().into();
    post.text = Set(req.text);

    match post.update(&db).await {
        Ok(post) => {
            Ok(warp::reply::with_status(warp::reply::json(&post), StatusCode::OK).into_response())
        }
        Err(_e) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn delete(
    id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();

    let post = post::Entity::find_by_id(id).one(&db).await;

    if post.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    let post = post.unwrap();

    if post.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    match Post::delete_by_id(id).exec(&db).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}
