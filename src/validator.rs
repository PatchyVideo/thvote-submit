
use std::collections::HashSet;

use actix_web::Error;
use bson::doc;
use futures_util::TryStreamExt;
use mongodb::Collection;

use crate::{models::{self, *}, shared::ServiceError};

#[derive(Debug, Clone)]
pub struct SubmitValidatorV1 {

}

impl SubmitValidatorV1 {
	pub fn new() -> Self {
		Self {

		}
	}
	pub async fn validate_character(&self, mut data: models::CharacterSubmitRest, coll: &Collection<CharacterSubmitRest>) -> Result<models::CharacterSubmitRest, ServiceError> {

		// step 2: retrieve and check if user attempts are allowed

		// first we lock submit for this vote_id
		let query = doc! {
			"meta.vote_id": data.meta.vote_id.clone()
		};
		let mut found_attempts = match coll.find(query, None).await {
			Ok(a) => a,
			Err(_) => { return Err(ServiceError::Unknown); }
		};
		let mut max_attempt: u32 = 0;
		while let Some(obj) = found_attempts.try_next().await.unwrap() {
			max_attempt = std::cmp::max(max_attempt, obj.meta.attempt.unwrap());
		};
		if max_attempt >= crate::shared::MAX_ATTEMPTS {
			return Err(ServiceError::TooManyAttempts);
		};
		let current_attempt = max_attempt + 1;
		data.meta.attempt = Some(current_attempt);
		// step 3: check ranks are unique from 1 to 6 and only one 本命
		let mut chset: HashSet<String> = HashSet::new();
		let mut first_set = false;
		for c in data.characters.iter() {
			if c.first.unwrap_or_default() {
				if first_set {
					return Err(ServiceError::InvalidContent);
				}
				first_set = true;
			}
			if chset.contains(&c.name) {
				return Err(ServiceError::InvalidContent);
			}
			chset.insert(c.name.clone());
		}
		// step 4: check all names are correct
		// step 5: return
		Ok(data)
	}
	pub async fn validate_music(&self, data: models::MusicSubmitRest, coll: &Collection<MusicSubmitRest>) -> Result<models::MusicSubmitRest, ServiceError> {
		todo!()
	}
	pub async fn validate_cp(&self, data: models::CPSubmitRest, coll: &Collection<CPSubmitRest>) -> Result<models::CPSubmitRest, ServiceError> {
		todo!()
	}
	pub async fn validate_work(&self, data: models::WorkSubmitRest, coll: &Collection<WorkSubmitRest>) -> Result<models::WorkSubmitRest, ServiceError> {
		todo!()
	}
	pub async fn validate_paper(&self, data: models::PaperSubmitRest, coll: &Collection<PaperSubmitRest>) -> Result<models::PaperSubmitRest, ServiceError> {
		todo!()
	}
}
