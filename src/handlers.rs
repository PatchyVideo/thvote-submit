
use actix_web::{web, App, HttpServer, Responder};
use bson::oid::ObjectId;
use actix_web::{error, http::header, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};

use serde::Serialize;

use crate::models;
use crate::shared::*;

pub const SUBMIT_VALIDATOR: &'static str = "127.0.0.1:1103";

type SubmitServiceV1Wrapper = web::Data<crate::services::SubmitServiceV1>;

pub async fn submit_character_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CharacterSubmitRest>) -> Result<web::Json<PostResult>, ServiceError> {
	let sanitized = service.validator.validate_character(body.0)?;
	service.submit_charcater(sanitized).await?;
	Ok(web::Json(PostResult::new()))
}

pub async fn submit_music_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::MusicSubmitRest>) -> Result<web::Json<PostResult>, ServiceError> {
	let sanitized = service.validator.validate_music(body.0)?;
	service.submit_music(sanitized).await?;
	Ok(web::Json(PostResult::new()))
}

pub async fn submit_cp_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CPSubmitRest>) -> Result<web::Json<PostResult>, ServiceError> {
	let sanitized = service.validator.validate_cp(body.0)?;
	service.submit_cp(sanitized).await?;
	Ok(web::Json(PostResult::new()))
}

pub async fn submit_work_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::WorkSubmitRest>) -> Result<web::Json<PostResult>, ServiceError> {
	let sanitized = service.validator.validate_work(body.0)?;
	service.submit_work(sanitized).await?;
	Ok(web::Json(PostResult::new()))
}

pub async fn submit_paper_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::PaperSubmitRest>) -> Result<web::Json<PostResult>, ServiceError> {
	let sanitized = service.validator.validate_paper(body.0)?;
	service.submit_paper(sanitized).await?;
	Ok(web::Json(PostResult::new()))
}