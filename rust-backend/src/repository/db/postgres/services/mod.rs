use std::{
    collections::{BTreeMap, HashMap},
    vec,
};

use packets::types;

use anyhow::anyhow;
use regex::bytes;
use sqlx::PgPool;

use crate::{domain, repository::db::postgres::packets::types::PacketDirection};

use super::packets;

#[derive(Clone)]
pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub fn new(db: PgPool) -> Self {
        Repository { db }
    }

    pub async fn upsert_service(&self, service: domain::Service) -> Result<i64, anyhow::Error> {
        // TODO: ВНИМАНИЕ, тут перед INSERT инкрементится поле id.
        let record = sqlx::query!(
            r#"
        INSERT INTO services(name, port)
            VALUES($1, $2)
                ON CONFLICT ON CONSTRAINT services_port_key
	                DO UPDATE SET
				        name=EXCLUDED.name
        RETURNING id
        "#,
            service.name,
            service.port as u32 as i32,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(record.id)
    }

    pub async fn get_rules_by_service(
        &self,
        service_ids: Vec<i64>,
    ) -> Result<HashMap<domain::Service, Vec<domain::Rule>>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT 
            services.id AS service_id,
            services.name AS service_name,
            services.port AS service_port,
            rules.id AS rule_id,
            rules.name AS rule_name,
            rules.packet_direction AS "rule_packet_direction: types::PacketDirection",
            rules.regexp AS rule_regexp,
            rules.color AS rule_color
        FROM services
        LEFT JOIN service_id_to_rule_id AS str 
            ON services.id = str.service_id
        JOIN rules 
            ON str.rule_id = rules.id
        WHERE 
            services.id = ANY($1::bigint[])
        "#,
            service_ids as _
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(HashMap::new()),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut rules_by_service: HashMap<domain::Service, Vec<domain::Rule>> = HashMap::new();

        for record in records.into_iter() {
            let service = domain::Service {
                id: record.service_id,
                name: record.service_name,
                port: record.service_port as i32,
            };

            let rule = domain::Rule {
                id: record.rule_id,
                name: record.rule_name,
                packet_direction: record.rule_packet_direction.into(),
                regexp: bytes::Regex::new(&record.rule_regexp)?,
                color: record.rule_color,
            };

            rules_by_service
                .entry(service)
                .and_modify(|rules| rules.push(rule.clone()))
                .or_insert(vec![rule]);
        }

        Ok(rules_by_service)
    }

    pub async fn get_all_services(&self) -> Result<Vec<domain::Service>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT id, name, port
        FROM services
        "#
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(vec![]),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut services: Vec<domain::Service> = Vec::with_capacity(records.len());

        for record in records.into_iter() {
            services.push(domain::Service {
                id: record.id,
                name: record.name,
                port: record.port as i32,
            });
        }

        Ok(services)
    }

    pub async fn delete_service(&self, id: i64) -> Result<domain::Service, anyhow::Error> {
        let record = sqlx::query!(
            r#"
        DELETE FROM services
        WHERE id = $1
        RETURNING 
            id AS service_id,
            name AS service_name,
            port AS service_port
        "#,
            id,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(domain::Service {
            id: record.service_id,
            name: record.service_name,
            port: record.service_port,
        })
    }

    pub async fn get_services_with_rules(
        &self,
    ) -> Result<BTreeMap<domain::Service, Vec<domain::Rule>>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT
            services.id AS service_id,
            services.name AS service_name,
            services.port AS service_port,
            rules.id AS rule_id,
            rules.name AS rule_name,
            rules.packet_direction AS "rule_packet_direction: types::PacketDirection",
            rules."regexp" AS rule_regexp,
            rules.color AS rule_color
        FROM
            services
        LEFT JOIN service_id_to_rule_id ON services.id = service_id_to_rule_id.service_id
        LEFT JOIN rules ON service_id_to_rule_id.rule_id = rules.id;
        "#
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(BTreeMap::default()),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut res: BTreeMap<domain::Service, Vec<domain::Rule>> = BTreeMap::new();

        for record in records.into_iter() {
            let rule = domain::Rule {
                id: record.rule_id.map_or(0, |id| id),
                name: record.rule_name.map_or("".to_string(), |n| n),
                packet_direction: record.rule_packet_direction.unwrap().into(),
                regexp: record
                    .rule_regexp
                    .map_or(bytes::Regex::new(""), |regexp| bytes::Regex::new(&regexp))?,
                color: record.rule_color.map_or("".to_string(), |c| c),
            };

            // Значения должны возвращаться всегда.
            res.entry(domain::Service {
                id: record.service_id,
                name: record.service_name,
                port: record.service_port,
            })
            .and_modify(|rules| {
                if rule.id.ne(&0) {
                    rules.push(rule.clone());
                }
            })
            .or_insert({
                if rule.id.ne(&0) {
                    vec![rule]
                } else {
                    vec![]
                }
            });
        }

        Ok(res)
    }

    pub async fn upsert_rule(&self, rule: domain::Rule) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
        INSERT INTO rules(name, packet_direction, regexp, color)
            VALUES($1, $2, $3, $4)
                ON CONFLICT ON CONSTRAINT rules_pkey
	                DO UPDATE SET
				        name=EXCLUDED.name,
                        packet_direction=EXCLUDED.packet_direction,
                        regexp=EXCLUDED.regexp,
                        color=EXCLUDED.color
        "#,
            rule.name,
            // hmmm nice moment
            packets::types::PacketDirection::from(rule.packet_direction) as PacketDirection,
            rule.regexp.to_string(),
            rule.color,
        )
        .execute(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(())
    }

    pub async fn get_all_rules(&self) -> Result<Vec<domain::Rule>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT
                id,
                name,
                packet_direction AS "packet_direction: types::PacketDirection",
                regexp,
                color
        FROM rules
        "#
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(vec![]),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut rules: Vec<domain::Rule> = Vec::with_capacity(records.len());

        for record in records.into_iter() {
            rules.push(domain::Rule {
                id: record.id,
                name: record.name,
                packet_direction: record.packet_direction.into(),
                regexp: bytes::Regex::new(&record.regexp)?,
                color: record.color,
            });
        }

        Ok(rules)
    }

    pub async fn get_rules_by_ids(
        &self,
        ids: Vec<i64>,
    ) -> Result<Vec<domain::Rule>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
         SELECT
                id,
                name,
                packet_direction AS "packet_direction: types::PacketDirection",
                regexp,
                color
        FROM rules
        WHERE id = ANY($1)
        "#,
            ids as _
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(vec![]),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut rules: Vec<domain::Rule> = Vec::with_capacity(records.len());

        for record in records.into_iter() {
            rules.push(domain::Rule {
                id: record.id,
                name: record.name,
                packet_direction: record.packet_direction.into(),
                regexp: bytes::Regex::new(&record.regexp)?,
                color: record.color,
            });
        }

        Ok(rules)
    }

    pub async fn delete_rule(&self, id: i64) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
        DELETE FROM rules
        WHERE
            id = $1
        "#,
            id,
        )
        .execute(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(())
    }

    pub async fn create_service_id_to_rule_ids(
        &self,
        service_id: i64,
        rule_ids: Vec<i64>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
        INSERT INTO service_id_to_rule_id(service_id, rule_id)
            SELECT 
                $1,
                rule_id
            FROM UNNEST($2::BIGINT[]) AS rule_id
        ON CONFLICT DO NOTHING
        "#,
            service_id,
            rule_ids as _,
        )
        .execute(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_service_id_to_rule_ids(
        &self,
        service_id: i64,
        rule_ids: Vec<i64>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
        DELETE FROM service_id_to_rule_id
        WHERE 
            service_id = $1
        AND
            rule_id = ANY($2)
        "#,
            service_id,
            rule_ids as _,
        )
        .execute(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(())
    }
}
