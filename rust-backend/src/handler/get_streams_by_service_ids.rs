use std::collections::HashMap;
use std::vec;

use crate::domain;
use crate::handler::types::{self, RuleWithBorders};
use crate::handler::types::{AppError, Packet, StreamWithPackets};
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::types::{Rule, Stream};

pub async fn get_streams_by_service_ids(
    ctx: Extension<types::AppContext>,
    Json(req): Json<GetStreamsByServiceIDsRequest>,
) -> Result<Json<GetStreamsByServiceIDsResponse>, AppError> {
    let rules_by_service = ctx
        .services_repo
        .get_rules_by_service(req.service_ids)
        .await
        .map_err(AppError::InternalServerError)?;

    let services = rules_by_service.keys();

    let packets_by_stream = ctx
        .streams_repo
        .get_packets_by_stream(
            services.into_iter().map(|s| s.port).collect(),
            req.last_stream_id,
            // TODO: сделать limit настраиваемым.
            20,
        )
        .await
        .map_err(AppError::InternalServerError)?;

    let mut rules_by_service_port: HashMap<i32, Vec<domain::Rule>> =
        HashMap::with_capacity(rules_by_service.len());

    for (service, rules) in rules_by_service.clone() {
        rules_by_service_port.insert(service.port, rules);
    }

    let mut resp = GetStreamsByServiceIDsResponse {
        stream_with_packets: Vec::with_capacity(packets_by_stream.len()),
    };

    for (stream, packets) in packets_by_stream {
        let mut rules_map: HashMap<Rule, ()> = HashMap::new();
        let mut started_at: DateTime<Utc> = packets[0].at;
        let mut ended_at: DateTime<Utc> = Default::default();
        let mut handler_packets: Vec<Packet> = Vec::with_capacity(packets.len());

        packets.into_iter().for_each(|p| {
            let mut rules_with_borders: Vec<RuleWithBorders> = vec![];

            rules_by_service_port
                .get(&stream.service_port)
                .map_or(vec![], |rules| rules.to_owned())
                .into_iter()
                .for_each(|r| {
                    if r.packet_direction.ne(&p.direction) {
                        return;
                    }

                    r.regexp.find_iter(p.payload.as_bytes()).for_each(|t| {
                        let rule: Rule = r.to_owned().into();

                        rules_with_borders.push(RuleWithBorders {
                            rule: rule.clone(),
                            start: t.start() as i64,
                            end: t.end() as i64,
                        });

                        rules_map.insert(rule, ());
                    });
                });

            if p.at < started_at {
                started_at = p.at
            }

            if p.at > ended_at {
                ended_at = p.at
            }

            handler_packets.push(Packet {
                payload: p.payload,
                rules_with_borders,
                direction: p.direction.to_string(),
                at: p.at.to_string(),
            });
        });

        let service_name = &rules_by_service
            .keys()
            .find(|&s| s.port == stream.service_port)
            .map_or("", |s| &s.name);

        let mut unique_rules: Vec<Rule> = rules_map.keys().cloned().collect();
        unique_rules.sort_by(|a, b| a.id.cmp(&b.id));

        resp.stream_with_packets.push(StreamWithPackets {
            stream: Stream {
                id: stream.id,
                service_name: service_name.to_string(),
                service_port: stream.service_port,
                rules: unique_rules,
                started_at: started_at.to_string(),
                ended_at: ended_at.to_string(),
            },
            packets: handler_packets,
        })
    }

    Ok(Json(resp))
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetStreamsByServiceIDsRequest {
    pub service_ids: Vec<i32>,
    pub last_stream_id: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetStreamsByServiceIDsResponse {
    pub stream_with_packets: Vec<StreamWithPackets>,
}
