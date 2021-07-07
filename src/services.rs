use bson::{doc, Document, oid::ObjectId};
use mongodb::{results::InsertOneResult, Collection, Database};

use crate::{models, validator};
use crate::shared::ServiceError;

#[derive(Clone)]
pub struct SubmitServiceV1 {
    pub character_coll: Collection,
    pub music_coll: Collection,
    pub cp_coll: Collection,
    pub work_coll: Collection,
    pub paper_coll: Collection,
    pub validator: validator::SubmitValidatorV1
}

impl SubmitServiceV1 {
    pub fn new(db: Database) -> SubmitServiceV1 {
        SubmitServiceV1 { 
            character_coll: db.collection("raw_character"),
            music_coll: db.collection("raw_music"),
            cp_coll: db.collection("raw_cp"),
            work_coll: db.collection("raw_work"),
            paper_coll: db.collection("raw_paper"),
            validator: validator::SubmitValidatorV1::new()
        }
    }

    pub async fn submit_charcater(&self, verified_data: models::CharacterSubmitRest) -> Result<ObjectId, ServiceError> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.character_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(ServiceError::Unknown)
    }

    pub async fn submit_music(&self, verified_data: models::MusicSubmitRest) -> Result<ObjectId, ServiceError> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.music_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(ServiceError::Unknown)
    }
    
    pub async fn submit_cp(&self, verified_data: models::CPSubmitRest) -> Result<ObjectId, ServiceError> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.cp_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(ServiceError::Unknown)
    }

    pub async fn submit_work(&self, verified_data: models::WorkSubmitRest) -> Result<ObjectId, ServiceError> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.work_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(ServiceError::Unknown)
    }

    pub async fn submit_paper(&self, verified_data: models::PaperSubmitRest) -> Result<ObjectId, ServiceError> {
        let mut attempts: u32 = 0;
        while attempts < 3 {
            match self.paper_coll.insert_one(bson::to_document(&verified_data).unwrap(), None).await {
                Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
                Err(_) => attempts += 1,
            };
        };
        Err(ServiceError::Unknown)
    }
}
