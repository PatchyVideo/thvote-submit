
use actix_web::Error;

use crate::{models, shared::ServiceError};

#[derive(Debug, Clone)]
pub struct SubmitValidatorV1 {

}

impl SubmitValidatorV1 {
    pub fn new() -> Self {
        Self {

        }
    }
    pub fn validate_character(&self, data: models::CharacterSubmitRest) -> Result<models::CharacterSubmitRest, ServiceError> {
        todo!()
    }
    pub fn validate_music(&self, data: models::MusicSubmitRest) -> Result<models::MusicSubmitRest, ServiceError> {
        todo!()
    }
    pub fn validate_cp(&self, data: models::CPSubmitRest) -> Result<models::CPSubmitRest, ServiceError> {
        todo!()
    }
    pub fn validate_work(&self, data: models::WorkSubmitRest) -> Result<models::WorkSubmitRest, ServiceError> {
        todo!()
    }
    pub fn validate_paper(&self, data: models::PaperSubmitRest) -> Result<models::PaperSubmitRest, ServiceError> {
        todo!()
    }
}
