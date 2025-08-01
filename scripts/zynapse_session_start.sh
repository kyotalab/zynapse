#!/bin/bash
# Zynapse Development Session Starter
# Usage: ./zynapse_session_start.sh WEEK TASK STYLE TOT_MODE

set -e

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/chat_config.conf"

# Parameters
WEEK=${1:-"Week1"}
TASK=${2:-"Unknown-Task"}
STYLE_PLAN=${3:-"A"}
TOT_MODE=${4:-"No"}

# Create timestamp
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
SESSION_ID=$(date '+%Y%m%d_%H%M%S')

# Create session log directory
mkdir -p "../sessions"
SESSION_LOG="../sessions/session_${SESSION_ID}.log"

# Function to display banner
display_banner() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}🚀 ZYNAPSE DEVELOPMENT SESSION${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Function to analyze style complexity
analyze_style_complexity() {
    local style="$1"
    local tot_mode="$2"

    # Parse style percentages
    local style_a_pct=$(echo "$style" | grep -o 'A(\([0-9]*\)%)' | grep -o '[0-9]*' || echo "0")
    local style_b_pct=$(echo "$style" | grep -o 'B(\([0-9]*\)%)' | grep -o '[0-9]*' || echo "0")
    local style_c_pct=$(echo "$style" | grep -o 'C(\([0-9]*\)%)' | grep -o '[0-9]*' || echo "0")

    # Handle simple style notation (A, B, C)
    if [[ "$style" =~ ^[ABC]$ ]]; then
        case "$style" in
        "A")
            style_a_pct=100
            style_b_pct=0
            style_c_pct=0
            ;;
        "B")
            style_a_pct=0
            style_b_pct=100
            style_c_pct=0
            ;;
        "C")
            style_a_pct=0
            style_b_pct=0
            style_c_pct=100
            ;;
        esac
    fi

    # Calculate estimated messages
    local base_messages=0
    base_messages=$((base_messages + style_a_pct * STYLE_A_BASE / 100))
    base_messages=$((base_messages + style_b_pct * STYLE_B_BASE / 100))
    base_messages=$((base_messages + style_c_pct * STYLE_C_BASE / 100))

    # Add ToT overhead
    if [[ "$tot_mode" == "ToT" ]]; then
        base_messages=$((base_messages + TOT_COMPLEXITY))
    fi

    echo "$base_messages"
}

# Main execution
main() {
    display_banner

    # Analyze complexity
    ESTIMATED_MESSAGES=$(analyze_style_complexity "$STYLE_PLAN" "$TOT_MODE")

    # Display session info
    echo -e "${PURPLE}📋 Session Information${NC}"
    echo "   Week: $WEEK"
    echo "   Task: $TASK"
    echo "   Style Plan: $STYLE_PLAN"
    echo "   ToT Mode: $TOT_MODE"
    echo "   Estimated Messages: ~$ESTIMATED_MESSAGES"
    echo "   Session ID: $SESSION_ID"
    echo ""

    # Check if task fits in single chat
    if [ "$ESTIMATED_MESSAGES" -gt "$CRITICAL_THRESHOLD" ]; then
        echo -e "${YELLOW}⚠️  WARNING: Task may require multiple chats${NC}"
        echo "   Estimated: $ESTIMATED_MESSAGES > Threshold: $CRITICAL_THRESHOLD"
        echo "   Consider breaking into sub-tasks"
        echo ""
    fi

    # Generate ToT expert team if needed
    TOT_EXPERTS=""
    if [[ "$TOT_MODE" == "ToT" ]]; then
        case "$TASK" in
        *"Error"* | *"error"*)
            TOT_EXPERTS="Rust専門家, UX専門家, 堅牢性専門家"
            ;;
        *"Note"* | *"Data"* | *"Structure"*)
            TOT_EXPERTS="データモデル専門家, シリアライゼーション専門家, Zettelkasten専門家"
            ;;
        *"Search"* | *"Tantivy"*)
            TOT_EXPERTS="検索エンジン専門家, パフォーマンス専門家, UX専門家"
            ;;
        *"CLI"*)
            TOT_EXPERTS="CLI設計専門家, ユーザビリティ専門家, システム統合専門家"
            ;;
        *"TUI"*)
            TOT_EXPERTS="UIアーキテクト専門家, イベント処理専門家, パフォーマンス専門家"
            ;;
        *)
            TOT_EXPERTS="アルゴリズム専門家, システム設計専門家, 品質保証専門家"
            ;;
        esac
        echo -e "${CYAN}🧠 ToT Expert Team: $TOT_EXPERTS${NC}"
        echo ""
    fi

    # Environment check
    echo -e "${GREEN}🔍 Environment Check${NC}"
    if [ -f "../Cargo.toml" ]; then
        echo "   ✅ Rust project detected"
        cd .. && cargo check --quiet && echo "   ✅ Project compiles" || echo "   ⚠️  Compilation issues detected"
        cd scripts
    else
        echo "   ⚠️  Cargo.toml not found"
    fi

    if git rev-parse --git-dir >/dev/null 2>&1; then
        echo "   ✅ Git repository detected"
        CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
        echo "   📍 Current branch: $CURRENT_BRANCH"
    else
        echo "   ⚠️  Not in git repository"
    fi
    echo ""

    # Generate chat template
    echo -e "${CYAN}📝 Chat Template (Copy to AI Chat)${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    cat <<TEMPLATE
=== ZYNAPSE開発セッション開始 ===

## 🎯 タスクコンテキスト
**週**: $WEEK
**タスク**: $TASK
**スタイル計画**: $STYLE_PLAN
**ToTモード**: $TOT_MODE
**セッションID**: $SESSION_ID
**タイムスタンプ**: $TIMESTAMP

## 📊 セッションパラメータ
**推定複雑度**: 約$ESTIMATED_MESSAGES メッセージ
**品質ゲート**: fmt, clippy, check, test, doc
**言語**: 日本語での回答をお願いします
TEMPLATE

    if [[ "$TOT_MODE" == "ToT" ]]; then
        cat <<TEMPLATE

## 🧠 ToT専門家チーム
**専門家構成**: $TOT_EXPERTS
**協業ルール**:
1. 各専門家が専門分野の視点で段階的評価を提供
2. 専門家間の相互検証と調整
3. 重要な問題発見時は具体的改善提案と共に早期退席
4. 最適解についての最終合意形成
TEMPLATE
    fi

    cat <<TEMPLATE

## 🚀 開始準備完了
**開発環境**: 検証済み
**品質基準**: 適用済み
**監視**: アクティブ

受信確認と指定されたアプローチでの開始をお願いします。

=== テンプレート終了 ===
TEMPLATE

    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Log session start
    cat <<LOG >>"$SESSION_LOG"
[$TIMESTAMP] SESSION START
Week: $WEEK
Task: $TASK
Style: $STYLE_PLAN
ToT: $TOT_MODE
Estimated Messages: $ESTIMATED_MESSAGES
Expert Team: $TOT_EXPERTS
LOG

    # Usage instructions
    echo ""
    echo -e "${YELLOW}💡 次のステップ:${NC}"
    echo "   1. 上記のテンプレートをAIチャットにコピー"
    echo "   2. 使用量監視: ./chat_usage_monitor.sh [count] [complexity]"
    echo "   3. 必要時に引き継ぎ生成: ./generate_handoff.sh"
    echo "   4. セッションログ: $SESSION_LOG"
    echo ""
    echo -e "${GREEN}🎯 Happy coding!${NC}"
}

# Execute main function
main "$@"
