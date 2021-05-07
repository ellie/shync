use std::convert::Infallible;

use chrono::Utc;
use serde::Serialize;
use warp::{reply::Response, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddHistoryRequest {
    pub id: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub data: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncHistoryRequest {
    pub sync_ts: chrono::DateTime<chrono::FixedOffset>,
    pub history_ts: chrono::DateTime<chrono::FixedOffset>,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncHistoryResponse {
    pub history: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub reason: String,
}

impl Reply for ErrorResponse {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}

pub struct ErrorResponseStatus {
    pub error: ErrorResponse,
    pub status: warp::http::StatusCode,
}

impl Reply for ErrorResponseStatus {
    fn into_response(self) -> Response {
        warp::reply::with_status(self.error, self.status).into_response()
    }
}

impl ErrorResponse {
    pub fn with_status(self, status: warp::http::StatusCode) -> ErrorResponseStatus {
        ErrorResponseStatus {
            error: self,
            status,
        }
    }

    pub fn reply(reason: &str) -> ErrorResponse {
        Self {
            reason: reason.to_string(),
        }
    }
}

pub enum JSONReply<T> {
    Ok(T),
    Err(ErrorResponseStatus),
}

impl<T: Send + Serialize> Reply for JSONReply<T> {
    fn into_response(self) -> Response {
        match self {
            JSONReply::Ok(t) => warp::reply::json(&t).into_response(),
            JSONReply::Err(e) => e.into_response(),
        }
    }
}

pub type JSONResult<T> = Result<JSONReply<T>, Infallible>;
pub fn json_error<T>(e: ErrorResponseStatus) -> JSONResult<T> {
    return Ok(JSONReply::Err(e));
}

pub fn json<T>(t: T) -> JSONResult<T> {
    return Ok(JSONReply::Ok(t));
}

pub enum SimpleReply<T> {
    Ok(T),
    Err(ErrorResponseStatus),
}

impl<T: Reply> Reply for SimpleReply<T> {
    fn into_response(self) -> Response {
        match self {
            SimpleReply::Ok(t) => t.into_response(),
            SimpleReply::Err(e) => e.into_response(),
        }
    }
}

pub type SimpleReplyResult<T> = Result<SimpleReply<T>, Infallible>;
pub fn reply_error<T>(e: ErrorResponseStatus) -> SimpleReplyResult<T> {
    return Ok(SimpleReply::Err(e));
}

pub fn reply<T>(t: T) -> SimpleReplyResult<T> {
    return Ok(SimpleReply::Ok(t));
}

