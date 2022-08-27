# SPDX-License-Identifier: Apache-2.0
# Configuration and variables
SHELL = /bin/bash
true = 1
false = 0

# Targets for database
.PHONY: start-database
start-database:
	$$($(is_database_created)); \
	if [ $$? -eq $(false) ]; then \
		$$($(create_database)); \
	fi
	$$($(is_database_running)); \
	if [ $$? -eq $(false) ]; then \
		$$($(start_database)); \
	fi

.PHONY: stop-database
stop-database:
	$$($(is_database_running)); \
	if [ $$? -eq $(true) ]; then \
		$$($(stop_database)); \
	fi

.PHONY: reset-database
reset-database:
	$$($(is_database_created)); \
	if [ $$? -eq $(true) ]; then \
		$$($(reset_database)); \
		$$($(stop_database)); \
	fi

# Targets for the application (host)
.PHONY: start-app
start-app: start-database
	export $$(cat ./env/develop-host.env | xargs -I {} bash -c 'echo "{}"'); \
	cargo run --bin book-store-app

.PHONY: stop-app
stop-app:
	pkill book-store-ap

# Bash functions for database
is_database_created = \
is_database_created() { \
	docker container inspect book-store-db 2>&1 > /dev/null; \
	if [ $$? -eq 0 ]; then \
		return $(true); \
	else \
		return $(false); \
	fi; \
}; \
is_database_created

is_database_running = \
is_database_running() { \
	state=$$(docker inspect book-store-db -f '{{ .State.Running }}'); \
	if [ $$state == "true" ]; then \
		return $(true); \
	else \
		return $(false); \
	fi; \
}; \
is_database_running

create_database = \
create_database() { \
	docker compose --file ./docker/develop.yml --env-file ./env/develop-guest.env --project-directory . up database; \
}; \
create_database

start_database = \
reset_database() { \
	docker compose --file ./docker/develop.yml --env-file ./env/develop-guest.env --project-directory . start database; \
}; \
reset_database

stop_database = \
stop_database() { \
	docker container stop book-store-db 2>&1 > /dev/null; \
}; \
stop_database

reset_database = \
reset_database() { \
	docker compose --file ./docker/develop.yml --env-file ./env/develop-guest.env --project-directory . start migration; \
}; \
reset_database
