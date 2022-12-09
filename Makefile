export
OTEL_EXPORTER_OTLP_ENDPOINT = http://localhost:4317
OTEL_RESOURCE_ATTRIBUTES = service.name=my-server,service.version=1.0.0,service.instance.id=id_from_k8s,service.namespace=prod

.PHONY: test
test:
	cargo test

.PHONY: run
run:
	cargo run --bin app

.PHONY: run-release
run-release:
	cargo run --release --bin app

.PHONY: compose-up
compose-up:
	docker compose up --build -d --force-recreate
	docker compose logs -f

.PHONY: compose-down
compose-down:
	docker compose down --remove-orphans