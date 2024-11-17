use axum::{Extension, Json};
use serde::Deserialize;

use crate::handler::types::{AppContext, AppError, AppResponse};

pub async fn delete_service_to_rule(
    ctx: Extension<AppContext>,
    Json(req): Json<DeleteServiceToRuleRequest>,
) -> Result<AppResponse, AppError> {
    ctx.services_repo
        .delete_service_id_to_rule_id(req.service_id, req.rule_id)
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(AppResponse::OK)
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteServiceToRuleRequest {
    pub service_id: i64,
    pub rule_id: i64,
}
