
use std::fmt::format;

use bson;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::shared;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteTokenClaim {
	pub vote_id: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitMetadata {
	/// 投票人id
	pub vote_id: String,
	/// 这是第几次提交该问卷（由本程序生成，无需提交）
	pub attempt: Option<u32>,
	/// 提交时间
	pub created_at: bson::DateTime,
	/// 用户IP
	pub user_ip: String,
	/// 额外用户指纹信息
	pub additional_fingreprint: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub characters: Vec<CharacterSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSubmitRest {
	pub music: Vec<MusicSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSubmitRest {
	pub works: Vec<WorkSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPSubmitRest {
	pub cps: Vec<CPSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperSubmitRest {
	pub papers: serde_json::Map<String, serde_json::Value>,
	pub meta: SubmitMetadata
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub name_c: Option<String>,
	pub active: Option<String>,
	pub reason: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub first: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSubmit {
	pub name: String,
	pub reason: Option<String>
}

// 人物部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   charcaters:[{
//     name: '',
//     reason: '', // 理由
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// CP部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   cps:[{
//     char1: '',
//     char2: '',
//     active: '', // 主动方
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 音乐部分：
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   musics:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 作品部分：
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   works:[{
//     name: '',
//     reason: '',
//     rank: // [0, 6], 0本命
//   }, ...]
// }
// 问卷部分
// {
//   meta: {
//	   vote_id: 2021,
//     attempt: 1
//   },
//   items:[{
//     item: '' //问卷项代码
//     answer: '' //回答内容
//   }, ...]
// }
