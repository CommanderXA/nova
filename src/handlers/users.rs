use std::{convert::Infallible, sync::Arc};

use entity::{follower, user};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, ModelTrait, Set,
    Statement, TransactionTrait, QueryFilter,
};
use tokio::sync::Mutex;
use warp::{hyper::StatusCode, Reply};

use crate::{errors::db::DbError, models::user::FollowersOfUser};

pub async fn list(
    db_session: Arc<Mutex<DatabaseConnection>>,
    _id_from_token: i32,
) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of users
    let db = db_session.lock().await.to_owned();
    let users: Vec<user::Model> = user::Entity::find().all(&db).await.unwrap();

    Ok(warp::reply::json(&users))
}

pub async fn get_by_id(
    user_id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    // Just return a JSON object of user
    let db = db_session.lock().await.to_owned();
    let user = user::Entity::find_by_id(user_id).one(&db).await;

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
    _id_from_token: i32,
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
//     _id_from_token: i32,
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
//     _id_from_token: i32,
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

#[allow(unused)]
pub async fn check_user_by_id(
    db_session: Arc<Mutex<DatabaseConnection>>,
    id: i32,
) -> Result<(), DbError> {
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

pub async fn follow(
    user_id: i32,
    follower_id: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<StatusCode, Infallible> {
    let db = db_session.lock().await.to_owned();

    // Getting users
    let user = user::Entity::find_by_id(user_id).one(&db).await;
    if user.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let follower = user::Entity::find_by_id(follower_id).one(&db).await;
    if follower.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let user = user.unwrap();
    if user.is_none() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let follower = follower.unwrap();
    if follower.is_none() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let subscription = follower::Entity::find_by_id((user_id, follower_id))
        .one(&db)
        .await;
    if subscription.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // checking the user's will based on the records
    match subscription.unwrap() {
        // user wants to unsubscribe
        Some(item) => {
            // Modifying users as well
            let mut user: user::ActiveModel = user.unwrap().into();
            user.followers = Set(user.followers.take().unwrap() - 1);
            let mut follower: user::ActiveModel = follower.unwrap().into();
            follower.following = Set(follower.following.take().unwrap() - 1);

            // Starting the transaction
            let txn = db.begin().await.unwrap();

            item.delete(&txn).await.unwrap();
            user.update(&txn).await.unwrap();
            follower.update(&txn).await.unwrap();

            match txn.commit().await {
                Ok(_) => Ok(StatusCode::OK),
                Err(_e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        // user wants to subscribe
        None => {
            // Modifying users as well
            let mut user: user::ActiveModel = user.unwrap().into();
            user.followers = Set(user.followers.take().unwrap() + 1);
            let mut follower: user::ActiveModel = follower.unwrap().into();
            follower.following = Set(follower.following.take().unwrap() + 1);

            // Starting the transaction
            let txn = db.begin().await.unwrap();

            follower::ActiveModel {
                user_id: Set(user_id),
                follower_id: Set(follower_id),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .unwrap();

            user.update(&txn).await.unwrap();
            follower.update(&txn).await.unwrap();

            match txn.commit().await {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(_e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

pub async fn get_user_followers(
    user_id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    let db = db_session.lock().await.to_owned();

    // Getting followers
    let followers = user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "user".id, username FROM "user" 
                    LEFT JOIN follower ON follower.follower_id = "user".id
                    WHERE follower.user_id = $1
                    ORDER BY follower.created_at DESC"#,
            [user_id.into()],
        ))
        .into_model::<FollowersOfUser>()
        .all(&db)
        .await;

    if followers.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    let followers = followers.unwrap();

    Ok(warp::reply::json(&followers).into_response())
}

pub async fn get_user_following(
    user_id: i32,
    _id_from_token: i32,
    db_session: Arc<Mutex<DatabaseConnection>>,
) -> Result<warp::reply::Response, Infallible> {
    let db = db_session.lock().await.to_owned();

    // Getting following
    let following = user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "user".id, username FROM "user" 
                    LEFT JOIN follower ON follower.user_id = "user".id
                    WHERE follower.follower_id = $1
                    ORDER BY follower.created_at DESC"#,
            [user_id.into()],
        ))
        .into_model::<FollowersOfUser>()
        .all(&db)
        .await;

    if following.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    let following = following.unwrap();

    Ok(warp::reply::json(&following).into_response())
}
