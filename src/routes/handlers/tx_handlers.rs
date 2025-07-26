use crate::utils::{api_response, app_state};
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use sea_orm::ColumnTrait;


#[derive(MultipartForm)]
struct CreateTxModel {
    tx_type: Text<i32>,
    tx_hash: Text<String>,
    from_address: Text<String>,
    to_address: Text<String>,
    tx_memo: Text<String>,
    tx_amount: Text<String>,
    tx_fee: Text<String>,
    tx_status: Text<String>,
    tx_time: Text<String>,
}

#[derive(Serialize, Deserialize)]
struct TxModel {
    pub id: i32,
    pub block_id: i32,
    pub tx_hash: String,
    pub tx_type: i32,
    pub from_address: String,
    pub to_address: String,
    pub tx_memo: String,
    pub tx_amount: String,
    pub tx_fee: String,
    pub tx_status: String,
    pub tx_time: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[post("create-tx")]
pub async fn create_tx(
    app_state: web::Data<app_state::AppState>,
    tx_info: MultipartForm<CreateTxModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    let tx_entity = entities::tx_info::ActiveModel {
        tx_type:   Set(tx_info.tx_type.clone()), 
        from_address: Set(tx_info.from_address.clone()),
        to_address: Set(tx_info.to_address.clone()),
        tx_memo: Set(tx_info.tx_memo.clone()),
        tx_amount: Set(tx_info.tx_amount.clone().parse::<i32>().unwrap()),
        tx_fee: Set(tx_info.tx_fee.clone().parse::<i32>().unwrap()),
        tx_status: Set(tx_info.tx_status.clone()),
        tx_time: Set(tx_info.tx_time.clone()),
        tx_hash: Set(tx_info.tx_hash.clone()),
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    tx_entity 
        .save(&txn)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    txn.commit()
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "Transaction created successfully".to_owned()))
}

#[get("tx/{tx_id}")]
pub async fn one_tx(
    app_state: web::Data<app_state::AppState>,
    tx_id: web::Path<i32>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let tx_info: TxModel = entities::tx_info::Entity::find_by_id(tx_id.into_inner())
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .map(|tx| TxModel {
            id: tx.id,
            block_id: tx.block_id,
            tx_hash: tx.tx_hash.clone(),
            tx_type: tx.tx_type,
            from_address: tx.from_address.clone(),
            to_address: tx.to_address.clone(),
            tx_memo: tx.tx_memo.clone(),
            tx_amount: tx.tx_amount.to_string(),
            tx_fee: tx.tx_fee.to_string(),
            tx_status: tx.tx_status.clone(),
            tx_time: tx.tx_time.to_string(),
            created_at: tx.created_at,
            updated_at: tx.updated_at,
        })
        .ok_or(api_response::ApiResponse::new(
            404,
            "Tx not found".to_string(),
        ))?;

    let resp_str = serde_json::to_string(&tx_info)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, resp_str.to_string()))
}

#[get("all-txs")]
pub async fn all_txs(
    app_state: web::Data<app_state::AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let all_txs = entities::tx_info::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()    
        .map(|txs| {
            TxModel {
                id: txs.id,
                block_id: txs.block_id,
                tx_hash: txs.tx_hash.clone(),
                tx_type: txs.tx_type,
                from_address: txs.from_address.clone(),
                to_address: txs.to_address.clone(),
                tx_memo: txs.tx_memo.clone(),
                tx_amount: txs.tx_amount.to_string(),
                tx_fee: txs.tx_fee.to_string(),
                tx_status: txs.tx_status.clone(),
                tx_time: txs.tx_time.to_string(),
                created_at: txs.created_at,
                updated_at: txs.updated_at,
            }
        }).collect::<Vec<TxModel>>();

        let resp_str = serde_json::to_string(&all_txs)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, resp_str.to_owned()))
}

#[get("tx-by-block-id/{block_id}")]
pub async fn tx_by_block_id(
    app_state: web::Data<app_state::AppState>,
    block_id: web::Path<i32>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let txs = entities::tx_info::Entity::find()
        .filter(entities::tx_info::Column::BlockId.eq(block_id.into_inner()))
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|txs| {
            TxModel {
                id: txs.id,
                block_id: txs.block_id,
                tx_type: txs.tx_type,
                tx_hash: txs.tx_hash.clone(),
                from_address: txs.from_address.clone(),
                to_address: txs.to_address.clone(),
                tx_memo: txs.tx_memo.clone(),       
                tx_amount: txs.tx_amount.to_string(),       
                tx_fee: txs.tx_fee.to_string(),
                tx_status: txs.tx_status.clone(),   
                tx_time: txs.tx_time.to_string(),
                created_at: txs.created_at,
                updated_at: txs.updated_at,
            }
        }).collect::<Vec<TxModel>>();

    let resp_str = serde_json::to_string(&txs)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

Ok(api_response::ApiResponse::new(200, resp_str.to_owned()))
}

#[get("tx-by-user-id/{user_id}")]
pub async fn tx_by_user_id(
    app_state: web::Data<app_state::AppState>,
    user_id: web::Path<i32>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {         

    // get user wallet address from user_id
    let user_info = entities::user_info::Entity::find()
        .filter(entities::user_info::Column::Id.eq(user_id.into_inner()))
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    if user_info.is_none() {
        return Ok(api_response::ApiResponse::new(404, "User not found".to_string()));
    }
    let user_info_val = user_info.unwrap();
    let user_wallet_address = user_info_val.wallet_address;


    // get txs from user wallet address
    let txs = entities::tx_info::Entity::find()
        .filter(
            entities::tx_info::Column::FromAddress.eq(user_wallet_address.to_string())
            .or(entities::tx_info::Column::ToAddress.eq(user_wallet_address.to_string()))
        )
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|txs| {
            TxModel {
                id: txs.id, 
                block_id: txs.block_id,
                tx_type: txs.tx_type,
                tx_hash: txs.tx_hash.clone(),
                from_address: txs.from_address.clone(),
                to_address: txs.to_address.clone(),
                tx_memo: txs.tx_memo.clone(),
                tx_amount: txs.tx_amount.to_string(),
                tx_fee: txs.tx_fee.to_string(),
                tx_status: txs.tx_status.clone(),
                tx_time: txs.tx_time.to_string(),
                created_at: txs.created_at,
                updated_at: txs.updated_at,
            }
        }).collect::<Vec<TxModel>>();

    let resp_str = serde_json::to_string(&txs)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, resp_str.to_owned()))
}