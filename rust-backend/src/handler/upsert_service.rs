use axum::{Extension, Json};
use serde::Deserialize;

use crate::domain;
use crate::handler::types::{AppContext, AppError, AppResponse, Service};
use crate::sniffer::external_types::PORTS_TO_SNIFF;

pub async fn upsert_service(
    ctx: Extension<AppContext>,
    Json(req): Json<UpsertServiceRequest>,
) -> Result<AppResponse, AppError> {
    // TODO: wrap into transaction
    let service = req.service;
    let service_id = ctx
        .services_repo
        .upsert_service(domain::Service {
            id: 0,
            name: service.name,
            port: service.port,
        })
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    ctx.services_repo
        .create_service_id_to_rule_ids(service_id, req.rule_ids.clone())
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    PORTS_TO_SNIFF.lock().await.insert(service.port, ());

    Ok(AppResponse::CREATED)
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpsertServiceRequest {
    pub service: Service,
    pub rule_ids: Vec<i64>,
}
