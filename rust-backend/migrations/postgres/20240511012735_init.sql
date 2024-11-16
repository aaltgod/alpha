-- +goose Up

-- +goose StatementBegin

CREATE TYPE packet_direction AS ENUM('IN', 'OUT');

CREATE TABLE IF NOT EXISTS services (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    port INT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS rules (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    regexp TEXT NOT NULL,
    color TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS service_id_to_rule_id (
    id BIGSERIAL PRIMARY KEY,
    service_id BIGINT NOT NULL,
    rule_id BIGINT NOT NULL,
    CONSTRAINT service_id_rule_id_idx UNIQUE (service_id, rule_id)
);

CREATE TABLE IF NOT EXISTS packets (
    id BIGSERIAL PRIMARY KEY,
    direction packet_direction NOT NULL,
    payload TEXT NOT NULL,
    stream_id BIGINT NOT NULL,
    at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX packets_stream_id_idx ON packets (stream_id);

CREATE TABLE IF NOT EXISTS streams (
    id BIGSERIAL PRIMARY KEY,
    service_port INT NOT NULL
);

CREATE INDEX streams_service_port_idx ON streams (service_port);

-- +goose StatementEnd

-- +goose Down

-- +goose StatementBegin

DROP INDEX IF EXISTS streams_service_port_idx;

DROP TABLE IF EXISTS streams;

DROP TABLE IF EXISTS packets;

DROP TABLE IF EXISTS service_id_to_rule_id;

DROP TABLE IF EXISTS rules;

DROP TABLE IF EXISTS services;

DROP TYPE IF EXISTS packet_direction;

-- +goose StatementEnd