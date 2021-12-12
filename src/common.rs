
use actix_web::{HttpResponse, web};
use actix_web::{http::StatusCode, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};
use serde::{Serialize, Deserialize};
use thiserror::Error;

pub const VOTE_YEAR: i32 = 2021;
pub const MAX_ATTEMPTS: i32 = 10;

use mongodb::{Collection, Database};

pub static SERVICE_NAME: &'static str = "submit-handler";

#[cfg(debug_assertions)]
pub const MONGODB_URL: &str = "mongodb://localhost:27017/";

#[cfg(not(debug_assertions))]
pub const MONGODB_URL: &str = "mongodb://mongo:27017/";
