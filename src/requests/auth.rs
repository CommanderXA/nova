use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Authentication data
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Data to logout from a session
pub struct LogoutRequest {
    pub token: String,
}
