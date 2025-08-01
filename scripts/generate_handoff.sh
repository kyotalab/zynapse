#!/bin/bash
# Generate Comprehensive Handoff Documentation
# Usage: ./generate_handoff.sh TASK_NAME PROGRESS STYLE CODE_FILE

set -e

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/chat_config.conf"

# Parameters
TASK_NAME=${1:-"Unknown-Task"}
PROGRESS=${2:-"0%"}
CURRENT_STYLE=${3:-"A"}
CODE_FILE=${4:-""}

# Timestamp
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
HANDOFF_ID=$(date '+%Y%m%d_%H%M%S')

# Create handoffs directory
mkdir -p "../handoffs"
HANDOFF_FILE="../handoffs/handoff_${HANDOFF_ID}_${TASK_NAME//[^a-zA-Z0-9]/_}.md"

# Function to analyze current git state
get_git_state() {
    if git rev-parse --git-dir >/dev/null 2>&1; then
        echo "**Git Branch**: $(git branch --show-current 2>/dev/null || echo 'unknown')"
        echo "**Git Status**: $(git status --porcelain | wc -l) uncommitted changes"
        echo "**Last Commit**: $(git log -1 --oneline 2>/dev/null || echo 'No commits')"
    else
        echo "**Git State**: Not in repository"
    fi
}

# Function to analyze project state
get_project_state() {
    if [ -f "../Cargo.toml" ]; then
        cd ..
        echo "**Cargo Check**: $(cargo check --quiet 2>&1 && echo "✅ Pass" || echo "❌ Fail")"
        echo "**Cargo Clippy**: $(cargo clippy --quiet 2>&1 && echo "✅ Pass" || echo "❌ Fail")"
        echo "**Cargo Test**: $(cargo test --quiet 2>&1 && echo "✅ Pass" || echo "❌ Fail")"
        echo "**Cargo Format**: $(cargo fmt --check 2>&1 && echo "✅ Pass" || echo "❌ Fail")"
        cd scripts
    else
        echo "**Project State**: No Cargo.toml found"
    fi
}

# Function to extract code content
get_code_content() {
    local file="$1"
    if [ -n "$file" ] && [ -f "../$file" ]; then
        echo "\`\`\`rust"
        cat "../$file"
        echo "\`\`\`"
    elif [ -n "$file" ] && [ -f "$file" ]; then
        echo "\`\`\`rust"
        cat "$file"
        echo "\`\`\`"
    else
        echo "\`\`\`rust"
        echo "// コードファイルが指定されていないか、ファイルが見つかりません"
        echo "// 現在の動作コードをここに手動で追加してください"
        echo "\`\`\`"
    fi
}

# Function to generate ToT expert team for task
get_expert_team() {
    local task="$1"
    case "$task" in
    *"Error"* | *"error"*)
        echo "Rust専門家, UX専門家, 堅牢性専門家"
        ;;
    *"Note"* | *"Data"* | *"Structure"*)
        echo "データモデル専門家, シリアライゼーション専門家, Zettelkasten専門家"
        ;;
    *"Search"* | *"Tantivy"*)
        echo "検索エンジン専門家, パフォーマンス専門家, UX専門家"
        ;;
    *"CLI"*)
        echo "CLI設計専門家, ユーザビリティ専門家, システム統合専門家"
        ;;
    *"TUI"*)
        echo "UIアーキテクト専門家, イベント処理専門家, パフォーマンス専門家"
        ;;
    *)
        echo "アルゴリズム専門家, システム設計専門家, 品質保証専門家"
        ;;
    esac
}

# Function to suggest next steps based on progress
suggest_next_steps() {
    local progress_num=$(echo "$PROGRESS" | grep -o '[0-9]*' | head -1)

    if [ "${progress_num:-0}" -lt 30 ]; then
        echo "1. 基本実装構造の完成"
        echo "2. 必須エラーハンドリングの追加"
        echo "3. 初期単体テストの作成"
    elif [ "${progress_num:-0}" -lt 60 ]; then
        echo "1. 残りのコア機能の実装"
        echo "2. 包括的エラーハンドリングの追加"
        echo "3. テストカバレッジの拡張"
    elif [ "${progress_num:-0}" -lt 80 ]; then
        echo "1. 実装詳細の完成"
        echo "2. パフォーマンスの最適化"
        echo "3. 統合テストの追加"
    else
        echo "1. 最終テストと検証"
        echo "2. ドキュメントの完成"
        echo "3. 品質保証レビュー"
    fi
}

# Main handoff generation
generate_handoff() {
    cat >"$HANDOFF_FILE" <<'HANDOFF'
# 🔄 ZYNAPSE チャット移行引き継ぎ

## 📋 引き継ぎメタデータ
**引き継ぎID**: `HANDOFF_ID_PLACEHOLDER`
**タイムスタンプ**: TIMESTAMP_PLACEHOLDER
**生成者**: `generate_handoff.sh`

---

## 🎯 現在のタスクコンテキスト

**タスクID**: `TASK_NAME_PLACEHOLDER`
**進捗**: PROGRESS_PLACEHOLDER
**現在のスタイル**: CURRENT_STYLE_PLACEHOLDER

### タスク説明
> このタスクがZynapseプロジェクトで達成する内容の簡潔な説明

### 現在のフェーズ
> 現在の開発フェーズと作業中の内容を説明

---

## 📊 技術的状態

### 現在のコード状態
CODE_CONTENT_PLACEHOLDER

### プロジェクトヘルスチェック
PROJECT_STATE_PLACEHOLDER

### バージョン管理ステータス
GIT_STATE_PLACEHOLDER

---

## 🧠 開発コンテキスト

### 重要な設計決定
1. **決定1**: [理由] - [影響範囲]
2. **決定2**: [理由] - [影響範囲]
3. **決定3**: [理由] - [影響範囲]

> **注**: 開発中に行った実際の決定で更新してください

### 却下された代替案
- **代替案A**: 却下理由
- **代替案B**: 却下理由

### 現在の課題/ブロッカー
> 対応が必要な即座の課題やブロッカーを記述

---

## 🎯 直近の次ステップ

### 優先アクション
NEXT_STEPS_PLACEHOLDER

### 必要な専門家チーム（ToTの場合）
**推奨専門家**: EXPERT_TEAM_PLACEHOLDER

**なぜこれらの専門家か**: 
- 専門家1: [必要な特定の専門知識]
- 専門家2: [必要な特定の専門知識]  
- 専門家3: [必要な特定の専門知識]

---

## 🚀 継続指示

### 推奨次チャットスタイル
**スタイル**: [現在のニーズに基づくA/B/C]
**ToTモード**: [Yes/No - 理由説明]

### 新AIセッション用コンテキスト
```markdown
前回のチャットがTASK_NAME_PLACEHOLDER実装中にメッセージ制限に達しました。

**現在の状況**: 
- 進捗: PROGRESS_PLACEHOLDER
- スタイル: CURRENT_STYLE_PLACEHOLDER
- 最終状態: [簡潔な説明]

**直近の目標**: [次に必要な具体的アクション]

**コンテキスト**: [AIが現在の状態を理解するための必要情報]

**言語**: 日本語での回答をお願いします
```

---

## ⚡ 即座再開テンプレート

### 新チャット即座開始用
```markdown
# 🔄 前回チャットからの継続

## 前回チャットのコンテキスト
前回のチャットがTASK_NAME_PLACEHOLDER開発中にメッセージ制限に達しました。

## 現在のステータス  
**タスク**: TASK_NAME_PLACEHOLDER
**進捗**: PROGRESS_PLACEHOLDER
**スタイル**: CURRENT_STYLE_PLACEHOLDER
**引き継ぎID**: HANDOFF_ID_PLACEHOLDER

## 現在のコード状態
[現在の動作するコードを挿入]

## 直近の目標
[次に必要な具体的アクション]

**言語**: 日本語での回答をお願いします

[推奨アプローチ]で継続の準備ができています。
```

### 緊急用クイックスタート
```markdown
🚨 緊急継続

**タスク**: TASK_NAME_PLACEHOLDER
**進捗**: PROGRESS_PLACEHOLDER  
**最終動作コード**: [コードスニペット]
**緊急課題**: [重要な次のアクション]
**スタイル**: CURRENT_STYLE_PLACEHOLDER

**言語**: 日本語での回答をお願いします

即座継続: [具体的な指示]
```

---

## 🔧 トラブルシューティングガイド

### よくある問題と解決策
1. **コードがコンパイルしない**: [一般的な原因を確認]
2. **テストが失敗**: [デバッグアプローチ]
3. **パフォーマンス問題**: [プロファイリング手順]

### 復旧手順
- **コンテキスト喪失**: この引き継ぎドキュメントを使用
- **実装破損**: 最後の動作状態に復帰
- **依存関係競合**: Cargo.lockを確認

---

*Zynapse Chat Management System v1.0 で生成*
*引き継ぎファイル: `HANDOFF_FILE_PLACEHOLDER`*
HANDOFF

    # Replace placeholders
    sed -i.bak "s/HANDOFF_ID_PLACEHOLDER/$HANDOFF_ID/g" "$HANDOFF_FILE"
    sed -i.bak "s/TIMESTAMP_PLACEHOLDER/$TIMESTAMP/g" "$HANDOFF_FILE"
    sed -i.bak "s/TASK_NAME_PLACEHOLDER/$TASK_NAME/g" "$HANDOFF_FILE"
    sed -i.bak "s/PROGRESS_PLACEHOLDER/$PROGRESS/g" "$HANDOFF_FILE"
    sed -i.bak "s/CURRENT_STYLE_PLACEHOLDER/$CURRENT_STYLE/g" "$HANDOFF_FILE"
    sed -i.bak "s/EXPERT_TEAM_PLACEHOLDER/$(get_expert_team "$TASK_NAME")/g" "$HANDOFF_FILE"
    sed -i.bak "s|HANDOFF_FILE_PLACEHOLDER|$HANDOFF_FILE|g" "$HANDOFF_FILE"

    # Replace code content
    local code_content=$(get_code_content "$CODE_FILE")
    local project_state=$(get_project_state)
    local git_state=$(get_git_state)
    local next_steps=$(suggest_next_steps)

    # Use temporary files for multi-line replacements
    echo "$code_content" >/tmp/code_content.tmp
    echo "$project_state" >/tmp/project_state.tmp
    echo "$git_state" >/tmp/git_state.tmp
    echo "$next_steps" >/tmp/next_steps.tmp

    # Replace multi-line content
    sed -i.bak "/CODE_CONTENT_PLACEHOLDER/r /tmp/code_content.tmp" "$HANDOFF_FILE"
    sed -i.bak "/CODE_CONTENT_PLACEHOLDER/d" "$HANDOFF_FILE"

    sed -i.bak "/PROJECT_STATE_PLACEHOLDER/r /tmp/project_state.tmp" "$HANDOFF_FILE"
    sed -i.bak "/PROJECT_STATE_PLACEHOLDER/d" "$HANDOFF_FILE"

    sed -i.bak "/GIT_STATE_PLACEHOLDER/r /tmp/git_state.tmp" "$HANDOFF_FILE"
    sed -i.bak "/GIT_STATE_PLACEHOLDER/d" "$HANDOFF_FILE"

    sed -i.bak "/NEXT_STEPS_PLACEHOLDER/r /tmp/next_steps.tmp" "$HANDOFF_FILE"
    sed -i.bak "/NEXT_STEPS_PLACEHOLDER/d" "$HANDOFF_FILE"

    # Clean up
    rm -f "$HANDOFF_FILE.bak" /tmp/*.tmp
}

# Execute main generation
main() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}📄 引き継ぎドキュメント生成中${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    echo "📋 タスク: $TASK_NAME"
    echo "📊 進捗: $PROGRESS"
    echo "🎨 スタイル: $CURRENT_STYLE"
    echo "📁 コードファイル: ${CODE_FILE:-'指定なし'}"
    echo "🆔 引き継ぎID: $HANDOFF_ID"
    echo ""

    # Generate handoff document
    generate_handoff

    echo -e "${GREEN}✅ 引き継ぎドキュメント生成成功!${NC}"
    echo "📄 ファイル: $HANDOFF_FILE"
    echo ""

    # Show preview of key sections
    echo -e "${YELLOW}📋 クイックプレビュー:${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Extract and show the quick resume template
    echo -e "${CYAN}⚡ 即座再開テンプレート:${NC}"
    sed -n '/### 新チャット即座開始用/,/### 緊急用クイックスタート/p' "$HANDOFF_FILE" | head -15

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Usage instructions
    echo -e "${YELLOW}💡 次のステップ:${NC}"
    echo "   1. 生成された引き継ぎドキュメントを確認"
    echo "   2. '即座再開テンプレート'を新しいチャットにコピー"
    echo "   3. 具体的詳細でテンプレートをカスタマイズ"
    echo "   4. 新セッション開始: ./zynapse_session_start.sh"
    echo ""

    # Log the handoff creation
    local latest_session=$(ls -t ../sessions/session_*.log 2>/dev/null | head -1)
    if [ -n "$latest_session" ]; then
        echo "[$TIMESTAMP] HANDOFF GENERATED: $HANDOFF_FILE" >>"$latest_session"
    fi

    echo -e "${GREEN}🎯 引き継ぎ準備完了!${NC}"
}

# Execute main function
main
