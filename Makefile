VERSION := $(shell git describe --tags --always --dirty | sed 's/^v//')

.PHONY: version
version:
	@echo $(VERSION)

.PHONY: install
install:
	poetry install

.PHONY: t test
t test:
	poetry run pytest tests/
