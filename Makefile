.PHONY: all setup install-tools install-deps build test lint fmt check machete features-check features-prune tag push-tag release clean help

VERSION := $(shell grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2)
TAG := v$(VERSION)

all: check build

help:
	@echo "totp-cli - Команды управления проектом:"
	@echo ""
	@echo "  make setup          - Установка всех необходимых инструментов (clippy, rustfmt, cargo-machete, cargo-features-manager)"
	@echo "  make check          - Полный аудит проекта (test, clippy, fmt, machete, features-check)"
	@echo "  make test           - Запуск модульных тестов"
	@echo "  make lint           - Проверка линтером clippy и rustfmt"
	@echo "  make fmt            - Автоматическое форматирование кода"
	@echo "  make machete        - Поиск неиспользуемых крейтов (cargo-machete)"
	@echo "  make features-check - Анализ неиспользуемых фичей без изменений (cargo-features)"
	@echo "  make features-prune - Автоматическое удаление лишних фичей из Cargo.toml"
	@echo "  make build          - Сборка оптимизированного release бинарника"
	@echo "  make clean          - Очистить артефакты сборки target/"
	@echo ""
	@echo "  make tag            - Создать локальный Git-тег $(TAG) по версии из Cargo.toml"
	@echo "  make push-tag       - Отправить тег $(TAG) на GitHub (запустит GitHub Release)"
	@echo "  make release        - Полный цикл: проверки -> создание тега $(TAG) -> push на GitHub"

setup: install-tools

install-tools:
	@echo "==> Установка компонентов rustup (clippy, rustfmt)..."
	rustup component add clippy rustfmt
	@echo "==> Установка утилит cargo (cargo-machete, cargo-features-manager)..."
	cargo install cargo-machete cargo-features-manager --locked
	@echo "✅ Все необходимые инструменты для Makefile успешно установлены!"

install-deps: install-tools

test:
	cargo test --all-targets

lint:
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check

fmt:
	cargo fmt

machete:
	cargo machete

features-check:
	cargo features prune --dry-run

features-prune:
	cargo features prune

check: test lint machete features-check

build:
	cargo build --release

clean:
	cargo clean

tag:
	@echo "==> Создание Git-тега $(TAG)..."
	@if git rev-parse "$(TAG)" >/dev/null 2>&1; then \
		echo "❌ Ошибка: Тег $(TAG) уже существует!"; \
		exit 1; \
	fi
	git tag -a "$(TAG)" -m "Release $(TAG)"
	@echo "✅ Тег $(TAG) успешно создан локально."

push-tag:
	@echo "==> Отправка тега $(TAG) в origin..."
	git push origin "$(TAG)"
	@echo "🚀 Тег $(TAG) отправлен! GitHub Actions запустил сборку релиза."

release: check tag push-tag
