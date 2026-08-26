#!/usr/bin/env bash
set -euo pipefail

# Извлекаем версию из Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2)
TAG="v$VERSION"

echo "========================================="
echo "  totp-cli release automation: $TAG"
echo "========================================="

# 1. Проверка рабочего дерева Git
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Внимание: в рабочем дереве есть незакоммиченные изменения:"
    git status -s
    echo ""
    read -r -p "Закоммитить изменения перед созданием тега? (y/N): " CONFIRM
    if [[ "$CONFIRM" =~ ^[Yy]$ ]]; then
        read -r -p "Введите сообщение коммита: " MSG
        git add .
        git commit -m "$MSG"
    else
        echo "Отмена операции."
        exit 1
    fi
fi

# 2. Проверка существования тега
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "❌ Ошибка: тег '$TAG' уже существует в локальном Git!"
    exit 1
fi

# 3. Запуск проверок (тесты, линтер, форматирование, зависимости, фичи)
echo "🔍 [1/3] Проверка кода, зависимостей и фичей..."
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo machete
cargo features prune --dry-run

# 4. Создание аннотированного тега
echo "🏷️  [2/3] Создание Git-тега $TAG..."
git tag -a "$TAG" -m "Release $TAG"
echo "✅ Тег $TAG успешно создан локально."

# 5. Отправка на GitHub
echo "🚀 [3/3] Отправка тега $TAG на GitHub..."
git push origin "$TAG"

echo "========================================="
echo "🎉 Готово! Тег $TAG отправлен на GitHub."
echo "   GitHub Actions запустил сборку бинарников и публикацию релиза."
echo "========================================="
