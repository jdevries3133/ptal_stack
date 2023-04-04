SHELL := /bin/bash

ENV=source .env &&

build: tailwind-build
	cd client-leptos; trunk build
	cd client-tauri; cargo tauri build
	cargo build --release --bin server

dev:
	cargo runcc \
	  'cd client-tauri; cargo tauri dev' \
	  'cargo watch -x "run --bin server"' \
	  'make tailwind-dev'

# Run clippy in cargo watch; I usually keep this going in my terminal while I
# write code for nearly instant continuous feedback
clip:
	cargo watch -x clippy


# END Database ################################################################

DB_CONTAINER_NAME := "ptal_stack_sql"

start-db:
	$(ENV) docker run \
        --name $(DB_CONTAINER_NAME) \
        -e POSTGRES_DATABASE="$$POSTGRES_DB" \
        -e POSTGRES_USER="$$POSTGRES_USER" \
        -e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
        -v $(PWD)/server/initdb.sql:/docker-entrypoint-initdb.d/initdb.sql \
        -p 5432:5432 \
        -d \
        postgres:15

stop-db:
	docker kill $(DB_CONTAINER_NAME) || true
	docker rm $(DB_CONTAINER_NAME) || true

reset-db: stop-db
	make start-db

watch-db:
	docker logs -f $(DB_CONTAINER_NAME)

shell-db:
	$(ENV) PGPASSWORD=$$POSTGRES_PASSWORD \
		psql -U "$$POSTGRES_USER" -h 0.0.0.0 $$POSTGRES_DB

# END Database ################################################################
# Tailwind / npm ##############################################################

tailwind-dev:
	# Affected by https://github.com/tailwindlabs/tailwindcss/issues/9870
	# / https://github.com/tailwindlabs/tailwindcss/issues/8517
	npx tailwindcss \
		-c ./client-leptos/tailwind.config.js \
		-i ./client-leptos/tailwind.css \
		-o ./client-leptos/tailwind.generated.css \
		--watch

tailwind-build:
	# Affected by https://github.com/tailwindlabs/tailwindcss/issues/9870
	# / https://github.com/tailwindlabs/tailwindcss/issues/8517
	npx tailwindcss \
		-c ./client-leptos/tailwind.config.js \
		-i ./client-leptos/tailwind.css \
		-o ./client-leptos/tailwind.generated.css \
		--minify

# END Tailwind / npm ##########################################################
