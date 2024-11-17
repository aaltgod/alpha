use axum::{Extension, Json};
use serde::Deserialize;

use crate::handler::types::{AppContext, AppError, AppResponse};

pub async fn delete_rule(
    ctx: Extension<AppContext>,
    Json(req): Json<DeleteRuleRequest>,
) -> Result<AppResponse, AppError> {
    ctx.services_repo
        .delete_rule(req.rule_id)
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(AppResponse::OK)
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteRuleRequest {
    pub rule_id: i64,
}
