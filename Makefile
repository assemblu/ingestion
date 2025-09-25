SHELL := /bin/bash
export $(shell sed -n 's/^\([^#][^=]*\)=.*/\1/p' .env)

.PHONY: up down seed test lint ch q grafana

up:
	docker compose up -d --build
	# init ClickHouse
	docker exec ch clickhouse-client -nm -q "INCLUDE '/db/bootstrap.sql'"

down:
	docker compose down -v

seed:
	docker exec -it simfeed sh -c "python /app/simfeed.py --symbols $$SYMBOL_SET_SIZE --rate 200"

test:
	docker exec -it ch clickhouse-client -q "SELECT count() FROM mart.trades"

lint:
	cargo fmt --all || true

ch:
	docker exec -it ch clickhouse-client

q:
	docker exec -it ch clickhouse-client -q "$$Q"

grafana:
	echo "http://localhost:3000  user: admin  pass: admin"

