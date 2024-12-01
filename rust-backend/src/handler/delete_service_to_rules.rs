use axum::{Extension, Json};
use serde::Deserialize;

use crate::handler::types::{AppContext, AppError, AppResponse};

pub async fn delete_service_to_rules(
    ctx: Extension<AppContext>,
    Json(req): Json<DeleteServiceToRulesRequest>,
) -> Result<AppResponse, AppError> {
    ctx.services_repo
        .delete_service_id_to_rule_ids(req.service_id, req.rule_ids)
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(AppResponse::OK)
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteServiceToRulesRequest {
    pub service_id: i64,
    pub rule_ids: Vec<i64>,
}
