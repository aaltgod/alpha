use anyhow::anyhow;
use axum::{Extension, Json};
use regex::bytes;
use serde::Deserialize;

use crate::domain::{self, PacketDirection};
use crate::handler::types::{AppContext, AppError, AppResponse, Rule};

pub async fn create_rule(
    ctx: Extension<AppContext>,
    Json(req): Json<CreateRuleRequest>,
) -> Result<AppResponse, AppError> {
    let name = req.rule.name;
    let packet_direction =
        PacketDirection::from_str(&req.rule.packet_direction).ok_or(AppError::BadRequest(
            "Невалидное значение у поля `packet_direction`".to_string(),
            anyhow!("invalid packet_direction {:?}", &req.rule.packet_direction),
        ))?;
    let color = req.rule.color;

    let regexp = bytes::Regex::new(req.rule.regexp.as_str()).map_err(|e| {
        AppError::BadRequest(
            "Невалидное регулярное выражение".to_string(),
            anyhow!(e.to_string()),
        )
    })?;

    ctx.services_repo
        .create_rule(domain::Rule {
            id: 0,
            name,
            packet_direction,
            regexp,
            color,
        })
        .await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(AppResponse::CREATED)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct CreateRuleRequest {
    pub rule: Rule,
}
