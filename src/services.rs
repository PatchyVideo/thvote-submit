use bson::{doc, oid::ObjectId};
use futures_util::{TryStreamExt};
use mongodb::{Collection, Database};
use pvrustlib::ServiceError;
use redlock::RedLock;

use crate::models::{CPSubmitRest, CharacterSubmitRest, MusicSubmitRest, PaperSubmitRest, WorkSubmitRest, VotingStatus};
use crate::{models, validator};
use crate::common::{SERVICE_NAME};

#[derive(Clone)]
pub struct SubmitServiceV1 {
    pub character_coll: Collection<CharacterSubmitRest>,
    pub music_coll: Collection<MusicSubmitRest>,
    pub cp_coll: Collection<CPSubmitRest>,
    pub work_coll: Collection<WorkSubmitRest>,
    pub paper_coll: Collection<PaperSubmitRest>,
    pub validator: validator::SubmitValidatorV1,
    pub lock: RedLock
}

impl SubmitServiceV1 {
    pub async fn new(db: Database, lock: RedLock) -> SubmitServiceV1 {
        SubmitServiceV1 { 
            character_coll: db.collection_with_type::<CharacterSubmitRest>("raw_character"),
            music_coll: db.collection_with_type::<MusicSubmitRest>("raw_music"),
            cp_coll: db.collection_with_type::<CPSubmitRest>("raw_cp"),
            work_coll: db.collection_with_type::<WorkSubmitRest>("raw_work"),
            paper_coll: db.collection_with_type::<PaperSubmitRest>("raw_paper"),
            validator: validator::SubmitValidatorV1::new().await,
            lock: lock
        }
    }

    pub async fn submit_charcater(&self, verified_data: models::CharacterSubmitRest) -> Result<ObjectId, ServiceError> {
        match self.character_coll.insert_one(verified_data.clone(), None).await {
            Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
            Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
        }
    }

    pub async fn get_submit_charcater(&self, vote_id: String) -> Result<CharacterSubmitRest, ServiceError> {
        let stages = vec![
            doc!{"$match": {"meta.vote_id": vote_id}},
            doc!{"$sort": {"meta.created_at": -1}}
        ];
        let mut cursor = self.character_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.ok_or(ServiceError::new_not_found(SERVICE_NAME, Some("submit_charcater".into())))?;
        let mut submit: CharacterSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        submit.meta.additional_fingreprint = None;
        submit.meta.user_ip = "".to_string();
        submit.meta.vote_id = "".to_string();
        Ok(submit)
    }

    pub async fn submit_music(&self, verified_data: models::MusicSubmitRest) -> Result<ObjectId, ServiceError> {
        match self.music_coll.insert_one(verified_data.clone(), None).await {
            Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
            Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
        }
    }

    pub async fn get_submit_music(&self, vote_id: String) -> Result<MusicSubmitRest, ServiceError> {
        let stages = vec![
            doc!{"$match": {"meta.vote_id": vote_id}},
            doc!{"$sort": {"meta.created_at": -1}}
        ];
        let mut cursor = self.music_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.ok_or(ServiceError::new_not_found(SERVICE_NAME, Some("submit_music".into())))?;
        let mut submit: MusicSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        submit.meta.additional_fingreprint = None;
        submit.meta.user_ip = "".to_string();
        submit.meta.vote_id = "".to_string();
        Ok(submit)
    }

    
    pub async fn submit_cp(&self, verified_data: models::CPSubmitRest) -> Result<ObjectId, ServiceError> {
        match self.cp_coll.insert_one(verified_data.clone(), None).await {
            Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
            Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
        }
    }

    pub async fn get_submit_cp(&self, vote_id: String) -> Result<CPSubmitRest, ServiceError> {
        let stages = vec![
            doc!{"$match": {"meta.vote_id": vote_id}},
            doc!{"$sort": {"meta.created_at": -1}}
        ];
        let mut cursor = self.cp_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.ok_or(ServiceError::new_not_found(SERVICE_NAME, Some("submit_cp".into())))?;
        let mut submit: CPSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        submit.meta.additional_fingreprint = None;
        submit.meta.user_ip = "".to_string();
        submit.meta.vote_id = "".to_string();
        Ok(submit)
    }

    pub async fn submit_paper(&self, verified_data: models::PaperSubmitRest) -> Result<ObjectId, ServiceError> {
        match self.paper_coll.insert_one(verified_data.clone(), None).await {
            Ok(insert_result) => return Ok(insert_result.inserted_id.as_object_id().unwrap().clone()),
            Err(e) => { return Err(ServiceError::new(SERVICE_NAME, format!("{:?}", e))); },
        }
    }

    pub async fn get_submit_paper(&self, vote_id: String) -> Result<PaperSubmitRest, ServiceError> {
        let stages = vec![
            doc!{"$match": {"meta.vote_id": vote_id}},
            doc!{"$sort": {"meta.created_at": -1}}
        ];
        let mut cursor = self.paper_coll.aggregate(stages, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        let submit = cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.ok_or(ServiceError::new_not_found(SERVICE_NAME, Some("submit_paper".into())))?;
        let mut submit: PaperSubmitRest = bson::from_document(submit).map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
        submit.meta.additional_fingreprint = None;
        submit.meta.user_ip = "".to_string();
        submit.meta.vote_id = "".to_string();
        Ok(submit)
    }

    pub async fn get_voting_status(&self, vote_id: String) -> Result<VotingStatus, ServiceError> {
        let stages = vec![
            doc!{"$match": {"meta.vote_id": vote_id}},
            doc!{"$sort": {"meta.created_at": -1}}
        ];
        
        let ch = {
            let mut cursor = self.character_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
            cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
        };
        let music = {
            let mut cursor = self.music_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
            cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
        };
        let cp = {
            let mut cursor = self.cp_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
            cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
        };
        let paper = {
            let mut cursor = self.paper_coll.aggregate(stages.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
            cursor.try_next().await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?.is_some()
        };
        Ok(VotingStatus {
            characters: ch,
            musics: music,
            cps: cp,
            papers: paper
        })
    }
}
