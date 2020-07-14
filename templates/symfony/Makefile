.PHONY: all install help clean
COMPOSER_CREATE_RPOJECT=test -s composer.json || composer create-project --no-interaction symfony/skeleton .

## Build the app
all:
	@docker-compose build
	@docker-compose run --rm app sh -c "${COMPOSER_CREATE_RPOJECT}"

## Install the app
install:
	@docker-compose up -d

## Stop and remove docker services
clean:
	@docker-compose down --volumes
	@docker-compose rm --force

## ¯\_(ツ)_/¯
help:
	@printf "${COLOR_COMMENT}Usage:${COLOR_RESET}\n"
	@printf " make [target] [arg=\"val\"...]\n\n"
	@printf "${COLOR_COMMENT}Available targets:${COLOR_RESET}\n"
	@awk '/^[a-zA-Z\-\_0-9\@]+:/ { \
		helpLine = match(lastLine, /^## (.*)/); \
		helpCommand = substr($$1, 0, index($$1, ":")); \
		helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
		printf " ${COLOR_INFO}%-24s${COLOR_RESET} %s\n", helpCommand, helpMessage; \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)
