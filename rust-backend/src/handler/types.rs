use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::domain;
use crate::repository::db::postgres::{services as services_repo, streams as streams_repo};

#[derive(Clone)]
pub struct AppContext {
    pub services_repo: services_repo::Repository,
    pub streams_repo: streams_repo::Repository,
}

pub enum AppResponse {
    OK,
    CREATED,
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => StatusCode::OK.into_response(),
            Self::CREATED => StatusCode::CREATED.into_response(),
        }
    }
}

pub enum AppError {
    InternalServerError(anyhow::Error),
    BadRequest(String, anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Self::InternalServerError(e) => {
                error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Внутренняя ошибка".to_owned(),
                )
            }
            Self::BadRequest(msg, e) => {
                error!("{e}");
                (StatusCode::BAD_REQUEST, msg)
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub port: i32,
}

impl From<domain::Service> for Service {
    fn from(service: domain::Service) -> Self {
        Service {
            id: service.id,
            name: service.name,
            port: service.port,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Services(Vec<Service>);

impl From<Vec<domain::Service>> for Services {
    fn from(services: Vec<domain::Service>) -> Self {
        Services(services.into_iter().map(|s| s.into()).collect())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ServiceWithRules {
    pub service: Service,
    pub rules: Rules,
}

#[derive(Clone, Debug, Serialize)]
pub struct Packet {
    pub payload: Vec<TextWithColor>,
    pub direction: String,
    pub at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct TextWithColor {
    pub text: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Stream {
    pub id: i64,
    pub service_name: String,
    pub service_port: i32,
    pub rules: Vec<Rule>,
    pub started_at: String,
    pub ended_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct StreamWithPackets {
    pub stream: Stream,
    pub packets: Vec<Packet>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Rule {
    pub id: i64,
    pub name: String,
    pub packet_direction: String,
    pub regexp: String,
    pub color: String,
}

impl From<domain::Rule> for Rule {
    fn from(rule: domain::Rule) -> Self {
        Rule {
            id: rule.id,
            name: rule.name.to_owned(),
            packet_direction: rule.packet_direction.to_string(),
            regexp: rule.regexp.to_string(),
            color: rule.color.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Rules(Vec<Rule>);

impl From<Vec<domain::Rule>> for Rules {
    fn from(rules: Vec<domain::Rule>) -> Self {
        Rules(rules.into_iter().map(|r| r.into()).collect())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleWithBorders {
    pub rule: Rule,
    pub start: i64,
    pub end: i64,
}
