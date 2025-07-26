use crate::utils::{api_response, app_state};
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveModelTrait;
use sea_orm::{EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(MultipartForm)]
struct CreateBlockModel {
    chain_id: Text<String>,
    block_number: Text<String>,
    block_slot: Text<String>,
    block_time: Text<String>,
    block_hash: Text<String>,
    block_parent_hash: Text<String>,
    block_nonce: Text<String>,
    block_difficulty: Text<String>,
}

#[derive(Serialize, Deserialize)]
struct BlockModel {
    pub id: i32,
    pub block_hash: String,
    pub block_number: String,
    pub block_slot: String,
    pub block_time: String,
    pub block_parent_hash: String,
    pub block_nonce: String,
    pub block_difficulty: String,
    pub block_gas_limit: String,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}

#[post("create-block")]
pub async fn create_block(
    app_state: web::Data<app_state::AppState>,
    block_info: MultipartForm<CreateBlockModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    let post_entity = entities::block_info::ActiveModel {
        chain_id: Set(block_info.chain_id.clone()),
        block_number: Set(block_info.block_number.clone().parse::<i32>().unwrap()),
        block_slot: Set(block_info.block_slot.clone().parse::<i32>().unwrap()),
        block_time: Set(block_info.block_time.clone().parse::<i32>().unwrap()),
        block_hash: Set(block_info.block_hash.clone()),
        parent_hash: Set(block_info.block_parent_hash.clone()),
        nonce: Set(block_info.block_nonce.clone().parse::<i32>().unwrap()),
        difficulty: Set(block_info.block_difficulty.clone().parse::<i32>().unwrap()),
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    post_entity
        .save(&txn)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    txn.commit()
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "OKKK".to_owned()))
}

#[get("block/{block_id}")]
pub async fn one_block(
    app_state: web::Data<app_state::AppState>,
    block_id: web::Path<i32>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let block_info: BlockModel = entities::block_info::Entity::find_by_id(block_id.into_inner())
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .map(|block| BlockModel {
            id: block.id,
            block_hash: block.block_hash.clone(),
            block_number: block.block_number.to_string(),
            block_slot: block.block_slot.to_string(),
            block_time: block.block_time.to_string(),
            block_parent_hash: block.block_hash.clone(),
            block_nonce: block.block_hash.clone(),
            block_difficulty: block.block_hash.clone(),
            block_gas_limit: block.block_hash.clone(),
            created_at: block.created_at,
            user_id: 0,
        })
        .ok_or(api_response::ApiResponse::new(
            404,
            "Block not found".to_string(),
        ))?;

    let resp_str = serde_json::to_string(&block_info)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, resp_str.to_string()))
}

#[get("all-blocks")]
pub async fn all_blocks(
    app_state: web::Data<app_state::AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let all_blocks = entities::block_info::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()    
        .map(|blocks| {
            BlockModel {
                id: blocks.id,
                block_hash: blocks.block_hash.clone(),
                block_number: blocks.block_number.to_string(),
                block_slot: blocks.block_slot.to_string(),
                block_time: blocks.block_time.to_string(),
                block_parent_hash: blocks.block_hash.clone(),
                block_nonce: blocks.block_hash.clone(),
                block_difficulty: blocks.block_hash.clone(),
                block_gas_limit: blocks.block_hash.clone(),
                created_at: blocks.created_at,
                user_id: 0,
            }
        }).collect::<Vec<BlockModel>>();

    let resp_str = serde_json::to_string(&all_blocks)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, resp_str.to_owned()))
}
