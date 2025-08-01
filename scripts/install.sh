#!/bin/bash
# Installation script for Zynapse Chat Management Scripts
# Usage: ./install.sh

set -e

echo "🚀 Zynapse Chat Management Scripts をインストール中..."

# Check if we're in the right directory
if [ ! -f "chat_config.conf" ]; then
    echo "❌ エラー: scriptsディレクトリからこのスクリプトを実行してください"
    echo "必要なファイル: chat_config.conf が見つかりません"
    exit 1
fi

# Make all scripts executable
echo "🔧 スクリプトを実行可能にしています..."
chmod +x *.sh

# Create necessary directories
echo "📁 必要なディレクトリを作成中..."
mkdir -p ../sessions ../handoffs ../logs

# Initialize learning log if it doesn't exist
if [ ! -f "../learning_log.md" ]; then
    echo "📝 learning_log.md を作成中..."
    cat >../learning_log.md <<'LOG'
# Zynapse Development Learning Log

開始日: $(date '+%Y-%m-%d %H:%M:%S')

## 概要

このログはZynapse Phase 1実装中の学習進捗、セッション結果、開発洞察を記録します。

---

LOG
    echo "✅ learning_log.md を作成しました"
fi

# Initialize git if not present
if ! git rev-parse --git-dir >/dev/null 2>&1; then
    echo "🔧 gitリポジトリを初期化中..."
    cd .. && git init && cd scripts

    # Create initial .gitignore
    cat >../.gitignore <<'GITIGNORE'
# Zynapse specific
/sessions/
/handoffs/
/logs/
*.log

# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
GITIGNORE
    echo "📄 .gitignore を作成しました"
fi

# Initialize Cargo project if needed
if [ ! -f "../Cargo.toml" ]; then
    echo "📦 Cargoプロジェクトを初期化中..."
    cd .. && cargo init --name zynapse && cd scripts
    echo "✅ Cargoプロジェクトを初期化しました"
fi

# Test all scripts
echo "🧪 スクリプトをテスト中..."

# Test configuration loading
if source "./chat_config.conf" 2>/dev/null; then
    echo "✅ 設定ファイルが正常に読み込まれます"
else
    echo "❌ 設定ファイルエラー"
    exit 1
fi

# Test script syntax
for script in *.sh; do
    if [ "$script" != "install.sh" ]; then
        if bash -n "$script"; then
            echo "✅ $script 構文OK"
        else
            echo "❌ $script 構文エラー"
            exit 1
        fi
    fi
done

echo ""
echo "🎉 インストール完了!"
echo ""
echo "📋 クイックスタートガイド:"
echo "   1. 実行: ./daily_development.sh"
echo "   2. インタラクティブプロンプトに従う"
echo "   3. 生成されたテンプレートをAIチャットにコピー"
echo "   4. 使用量監視: ./chat_usage_monitor.sh [count] [complexity]"
echo ""
echo "📚 詳細な使用方法: README.md を参照"
echo "🔧 設定カスタマイズ: chat_config.conf を編集"
echo ""
echo "🚀 Zynapse開発を楽しんでください!"
