use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateRequest {
    pub related_to_post: Option<i32>,
    pub user_id: i32,
    pub text: String,
}
