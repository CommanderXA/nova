use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(FromQueryResult, Serialize)]
pub struct FollowersOfUser {
    pub id: i32,
    pub username: String,
}