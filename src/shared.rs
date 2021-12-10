
use actix_web::{HttpResponse, web};
use actix_web::{http::StatusCode, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};
use serde::{Serialize, Deserialize};
use thiserror::Error;

pub const VOTE_YEAR: i32 = 2021;
pub const MAX_ATTEMPTS: i32 = 10;

#[derive(Serialize)]
pub struct ErrorResponse {
	code: u16,
	error: String,
	message: String,
}

#[derive(Error, Debug)]
pub enum ServiceError {
	#[error("Invalid form content")]
	InvalidContent { reason: String },
	#[error("NotFound")]
	NotFound,
	#[error("Forbidden")]
	Forbidden,
	#[error("Too many attempts")]
	TooManyAttempts,
	#[error("Unknown Internal Error")]
	Unknown,
}
impl ServiceError {
	pub fn name(&self) -> String {
		match self {
			Self::InvalidContent { reason } => "InvalidContent".to_string(),
			Self::TooManyAttempts => "TooManyAttempts".to_string(),
			Self::NotFound => "NotFound".to_string(),
			Self::Forbidden => "Forbidden".to_string(),
			Self::Unknown => "Unknown".to_string(),
		}
	}
}
impl ResponseError for ServiceError {
	fn status_code(&self) -> StatusCode {
		match *self {
			Self::InvalidContent { reason: _ } => StatusCode::UNPROCESSABLE_ENTITY,
			Self::TooManyAttempts => StatusCode::TOO_MANY_REQUESTS,
			Self::NotFound => StatusCode::NOT_FOUND,
			Self::Forbidden => StatusCode::FORBIDDEN,
			Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}

	fn error_response(&self) -> HttpResponse {
		let status_code = self.status_code();
		let error_response = ErrorResponse {
			code: status_code.as_u16(),
			message: self.to_string(),
			error: self.name(),
		};
		HttpResponse::build(status_code).json(error_response)
	}
}


#[derive(Serialize)]
pub struct PostResult {
	code: i32,
	message: String,
}
impl PostResult {
	pub fn successful_submit() -> PostResult {
		PostResult {
			code: 200,
			message: "提交成功".to_string()
		}
	}
}

use mongodb::{Collection, Database};

#[cfg(debug_assertions)]
pub const MONGODB_URL: &str = "mongodb://localhost:27017/";

#[cfg(not(debug_assertions))]
pub const MONGODB_URL: &str = "mongodb://mongo:27017/";
