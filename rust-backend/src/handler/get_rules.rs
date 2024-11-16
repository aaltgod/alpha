use axum::{Extension, Json};
use serde::Serialize;

use crate::handler::types::{AppContext, AppError};

use super::types::Rules;

pub async fn get_rules(ctx: Extension<AppContext>) -> Result<Json<GetRulesResponse>, AppError> {
    let rules = ctx
        .services_repo
        .get_all_rules()
        .await
        .map_err(AppError::InternalServerError)?;

    Ok(Json(GetRulesResponse {
        rules: Rules::from(rules),
    }))
}

#[derive(Clone, Debug, Serialize)]
pub struct GetRulesResponse {
    pub rules: Rules,
}
