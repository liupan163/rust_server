use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use sea_orm::{ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize, Deserialize)]
struct UpdateUserInfo {
    name: String,
    age: i32,
    image: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    age: i32,
    image: String,
    password: String,
    wallet_address: String,
}

#[get("my_info")]
pub async fn my_info(
    app_state: web::Data<app_state::AppState>,
    claims: web::ReqData<Claims>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let user_model = entities::user_info::Entity::find_by_id(claims.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User not found".to_string(),
        ))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!(
            " {{ 'name': '{}', 'age': '{}', 'image': '{}' }} ",
            user_model.name, user_model.age, user_model.image
        ),
    ))
}

#[post("update")]
pub async fn update_user_info(
    data: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserInfo>,
    claims: web::ReqData<Claims>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let mut user_model = entities::user_info::Entity::find_by_id(claims.id)
        .one(&data.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User not found".to_string(),
        ))?
        .into_active_model();

    user_model.name = Set(user_data.name.clone());
    user_model.age = Set(user_data.age);
    user_model.image = Set(user_data.image.clone());

    user_model
        .update(&data.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "success".to_string()))
}

#[post("register")]
pub async fn register(
    data: web::Data<app_state::AppState>,
    user_data: web::Json<RegisterModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    // check if user already exists
    let user_exists = entities::user_info::Entity::find()
        .filter(Condition::all().add(entities::user_info::Column::Email.eq(&user_data.email)))
        .one(&data.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    if user_exists.is_some() {
        return Ok(api_response::ApiResponse::new(
            400,
            "User already exists".to_string(),
        ));
    }

    // create user
    let user_model = entities::user_info::ActiveModel {
        name: Set(user_data.name.clone()),
        age: Set(user_data.age),
        image: Set(user_data.image.clone()),
        email: Set(user_data.email.clone()),
        password: Set(digest(&user_data.password)),
        wallet_address: Set(user_data.wallet_address.clone()),
        ..Default::default()
    }
    .insert(&data.db)
    .await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{}", user_model.id),
    ))
}
