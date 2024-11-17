use axum::{Extension, Json};
use serde::Deserialize;

use crate::{
    handler::types::{AppContext, AppError, AppResponse},
    sniffer::external_types::PORTS_TO_SNIFF,
};

pub async fn delete_service(
    ctx: Extension<AppContext>,
    Json(req): Json<DeleteServiceRequest>,
) -> Result<AppResponse, AppError> {
    let service = ctx
        .services_repo
        .delete_service(req.service_id)
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    PORTS_TO_SNIFF.lock().await.remove(&service.port);

    Ok(AppResponse::OK)
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteServiceRequest {
    pub service_id: i64,
}
