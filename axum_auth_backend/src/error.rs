// This file contains all our error custom types, error responses and helper functions to manage errorr 
use axum{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage{
    EmptyPassword,
    ExcededMaxPasswordLength(usize),
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNolomgerExists,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
    InvalidHashFormat,
}

impl ToString for ErrorMessage{
    fn to_string(&self) -> String{
        self.to_str().to_owned()
    }
}

impl ErrorMessage{
    fn to_str(&self) -> String{
        match self{
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExcededMaxPasswordLength(length: &usize) => format!("Password must be at most {} characters long.", length),
            ErrorMessage::HashingError => "Error hashing password".to_string(),
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            ErrorMessage::ServerError => "Internal server error".to_string(),
            ErrorMessage::WrongCredentials => "Wrong email or password".to_string(),
            ErrorMessage::EmailExists => "Email already exists".to_string(),
            ErrorMessage::UserNolomgerExists => "User no longer exists".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
        }
    }
}

pub struct HttpError{
    pub status: StatusCode,
    pub message: String,
}

impl HttpError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        HttpError { 
            status, 
            message: message.into(), 
        }
    }
}

pub fn server_error(message: impl Into<String>) -> self {
    HttpError{
        message: message.into(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn bad_request(message: impl Into<String>) -> self {
    HttpError {
        message: message.into(),
        status: StatusCode::BAD_REQUEST,
    }
}

pub fn unique_constraint_violation(message: impl Into<String>) -> self {
    HttpError {
        message: message.into(),
        status: StatusCode::CONFLICT,
    }
}

pub fn unauthorized(message: impl Into<String>) -> self {
    HttpError {
        message: message.into(),
        status: StatusCode::UNAUTHORIZED,
    }
}

pub fn into_http_response(self) -> Response {
    let json_response = json(ErrorResponse) {
        status: "fail"to_string(),
        message: message.clone(),
    };
    
    (self.status, json_response(error_response)).into_response()
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}", self.message, self.status
    )}
}

impl std::error::Error for HttpError {
    
}