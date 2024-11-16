use anyhow::anyhow;
use axum::{Extension, Json};
use regex::bytes;
use serde::Deserialize;

use crate::domain;
use crate::handler::types::{AppContext, AppError, AppResponse, Rule};

pub async fn upsert_rule(
    ctx: Extension<AppContext>,
    Json(req): Json<UpsertRuleRequest>,
) -> Result<AppResponse, AppError> {
    let name = req.rule.name;
    let color = req.rule.color;

    let regexp = bytes::Regex::new(req.rule.regexp.as_str()).map_err(|e| {
        AppError::BadRequest(
            "Невалидное регулярное выражение".to_string(),
            anyhow!(e.to_string()),
        )
    })?;

    ctx.services_repo
        .upsert_rule(domain::Rule {
            id: 0,
            name,
            regexp,
            color,
        })
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(AppResponse::CREATED)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct UpsertRuleRequest {
    pub rule: Rule,
}
