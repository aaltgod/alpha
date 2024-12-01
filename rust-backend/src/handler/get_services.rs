use axum::{Extension, Json};
use serde::Serialize;

use crate::handler::types::{AppContext, AppError, ServiceWithRules};

pub async fn get_services(
    ctx: Extension<AppContext>,
) -> Result<Json<GetServicesResponse>, AppError> {
    let services_with_rules = ctx
        .services_repo
        .get_services_with_rules()
        .await
        .map_err(AppError::InternalServerError)?;

    let res = services_with_rules
        .into_iter()
        .map(|(service, rules)| ServiceWithRules {
            service: service.into(),
            rules: rules.into(),
        })
        .collect();

    Ok(Json(GetServicesResponse {
        services_with_rules: res,
    }))
}

#[derive(Clone, Debug, Serialize)]
pub struct GetServicesResponse {
    pub services_with_rules: Vec<ServiceWithRules>,
}
