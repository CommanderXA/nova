use std::{convert::Infallible, sync::Arc};

use sea_orm::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::sync::Mutex;
use warp::{hyper::StatusCode, reject, Reply};

use entity::session;
use entity::session::Entity as Session;
use entity::user;
use entity::user::Entity as User;

use crate::models::role::Role;
use crate::{
    errors::{db::DbError, jwt::JWTError},
    filters::auth::check_token,
    jwt::generate_jwt,
    requests::auth::{AuthRequest, LogoutRequest},
};

use super::users::create;

pub async fn login(
    session: Arc<Mutex<DatabaseConnection>>,
    body: AuthRequest,
) -> Result<warp::reply::Response, Infallible> {
    let result = validate_user(session, body).await;

    match result {
        Ok(token) => Ok(warp::reply::json(&token).into_response()),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()),
    }
}

pub async fn register(
    db_session: Arc<Mutex<DatabaseConnection>>,
    body: AuthRequest,
) -> Result<warp::reply::Response, Infallible> {
    let user = user::ActiveModel {
        username: Set(body.username.clone()),
        email: Set(body.email.clone()),
        password: Set(body.password.clone()),
        ..Default::default()
    };

    let created = create(user, db_session.clone()).await.unwrap();

    if !created.is_success() {
        return Ok(created.into_response());
    }

    let result = validate_user(db_session, body).await;

    log::error!("{result:?}");
    match result {
        Ok(token) => Ok(warp::reply::json(&token).into_response()),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()),
    }
}

pub async fn logout(
    db_session: Arc<Mutex<DatabaseConnection>>,
    body: LogoutRequest,
) -> Result<warp::reply::Response, Infallible> {
    let _token_decoded = check_token(db_session.clone(), body.token.clone())
        .await
        .map_err(|_| reject::custom(JWTError::JWTTokenError))
        .unwrap();

    let db = db_session.lock().await.to_owned();
    let result = session::Entity::delete_by_id(body.token).exec(&db).await;

    match result {
        Ok(_) => Ok(StatusCode::UNAUTHORIZED.into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn validate_user(
    db_session: Arc<Mutex<DatabaseConnection>>,
    body: AuthRequest,
) -> Result<String, DbError> {
    let db = db_session.lock().await.to_owned();

    let user: Option<user::Model> = User::find()
        .filter(user::Column::Username.contains(&body.username))
        .one(&db)
        .await
        .unwrap();

    if user.is_none() {
        return Err(DbError::WrongCredentials);
    }

    let user = user.unwrap();

    if user.password != body.password {
        return Err(DbError::WrongCredentials);
    }

    add_jwt_session(db_session, &user).await
}

pub async fn add_jwt_session(
    db_session: Arc<Mutex<DatabaseConnection>>,
    user: &user::Model,
) -> Result<String, DbError> {
    let user = user.to_owned();

    let token = generate_jwt(
        user.id,
        Role::from_u8(user.role.clone() as u8).unwrap(),
    );
    let token = token.unwrap();

    let db = db_session.lock().await.to_owned();
    let user_session = session::ActiveModel {
        jwt: Set(token.clone()),
        user_id: Set(user.id),
        ..Default::default()
    };
    let res = session::Entity::insert(user_session).exec(&db).await;

    if res.is_err() {
        return Err(DbError::FailedToAdd);
    }

    Ok(token)
}

pub async fn validate_session(
    db_session: Arc<Mutex<DatabaseConnection>>,
    token: &str,
) -> Result<(), DbError> {
    let db = db_session.lock().await.to_owned();
    let result = Session::find_by_id(token).one(&db).await;

    if result.is_err() {
        return Err(DbError::InternalError);
    }

    if result.unwrap().is_none() {
        return Err(DbError::NotFound);
    }

    Ok(())
}
