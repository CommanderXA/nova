use serde::Serialize;

#[allow(unused)]
#[derive(Debug, Serialize)]
pub enum DbError {
    AlreadyExists,
    WrongCredentials,
    FailedToConvertRow,
    FailedToAdd,
    NotFound,
    InternalError,
}
