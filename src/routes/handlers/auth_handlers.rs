use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state};
use actix_web::{post, web};
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;

#[derive(Serialize, Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_data = entities::user_info::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user_info::Column::Email.eq(&login_json.email))
                .add(entities::user_info::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User Not Found".to_owned()))?;

    // generate jwt token and return it
    let token = encode_jwt(user_data.email, user_data.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{{ 'token':'{}' }}", token),
    ))
}
