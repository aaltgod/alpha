use axum::{Extension, Json};
use serde::Serialize;

use crate::handler::types::{AppContext, AppError, Services};

pub async fn get_services(
    ctx: Extension<AppContext>,
) -> Result<Json<GetServicesResponse>, AppError> {
    let services = ctx
        .services_repo
        .get_all_services()
        .await
        .map_err(AppError::InternalServerError)?;

    Ok(Json(GetServicesResponse {
        services: Services::from(services),
    }))
}

#[derive(Clone, Debug, Serialize)]
pub struct GetServicesResponse {
    pub services: Services,
}
