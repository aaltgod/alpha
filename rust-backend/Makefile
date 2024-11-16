include .env
export

APP_NAME=rust-backend

.PHONY: run
run: compose-up build
	@sleep 1
	$(info "RUN $(APP_NAME)")

	sudo RUST_LOG=warn ./target/release/$(APP_NAME)

.PHONY: build
build:
	$(info "BUILD")
	cargo build --release

.PHONY:
clean: compose-down
	$(info "CLEAN")
	rm -rf bin || true

install-goose:
	$(info go install github.com/pressly/goose/v3/cmd/goose@latest)
	go install github.com/pressly/goose/v3/cmd/goose@latest

LOCAL_MIGRATION_DIR=./migrations/postgres
LOCAL_MIGRATION_DSN="host=${POSTGRES_HOST} port=${POSTGRES_PORT} dbname=${POSTGRES_DATABASE} user=${POSTGRES_USERNAME} password=${POSTGRES_PASSWORD} sslmode=disable"

create-new-migration:
	goose -dir ${LOCAL_MIGRATION_DIR} postgres db create ${name} sql

migration-up:
	goose -dir ${LOCAL_MIGRATION_DIR} postgres ${LOCAL_MIGRATION_DSN} up -v

migration-down:
	goose -dir ${LOCAL_MIGRATION_DIR} postgres ${LOCAL_MIGRATION_DSN} down -v

migration-reset:
	goose -dir ${LOCAL_MIGRATION_DIR} postgres ${LOCAL_MIGRATION_DSN} reset

.PHONY: compose-up
compose-up:
	docker compose -p alpha -f ./docker-compose.yaml up -d

.PHONY: compose-stop
compose-stop:
	docker compose -p alpha -f ./docker-compose.yaml stop

.PHONY: compose-down
compose-down:
	docker compose -p alpha -f ./docker-compose.yaml down
