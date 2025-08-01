#!/bin/bash
# Chat Usage Monitor for Zynapse Development
# Usage: ./chat_usage_monitor.sh CURRENT_COUNT [COMPLEXITY]

set -e

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/chat_config.conf"

# Parameters
CURRENT_COUNT=${1:-0}
COMPLEXITY=${2:-"Medium"}

# Input validation
if ! [[ "$CURRENT_COUNT" =~ ^[0-9]+$ ]]; then
    echo "Error: CURRENT_COUNT must be a number"
    echo "Usage: $0 CURRENT_COUNT [COMPLEXITY]"
    exit 1
fi

# Function to get required messages for complexity
get_required_messages() {
    case "$1" in
    "Low" | "low") echo "$LOW_COMPLEXITY" ;;
    "Medium" | "medium") echo "$MEDIUM_COMPLEXITY" ;;
    "High" | "high") echo "$HIGH_COMPLEXITY" ;;
    "ToT" | "tot") echo "$TOT_COMPLEXITY" ;;
    *) echo "$MEDIUM_COMPLEXITY" ;;
    esac
}

# Function to determine zone
get_zone() {
    local usage_pct=$1
    if [ "$usage_pct" -lt "$WARNING_THRESHOLD" ]; then
        echo "GREEN"
    elif [ "$usage_pct" -lt "$CRITICAL_THRESHOLD" ]; then
        echo "YELLOW"
    elif [ "$usage_pct" -lt "$EMERGENCY_THRESHOLD" ]; then
        echo "RED"
    else
        echo "CRITICAL"
    fi
}

# Function to display zone status
display_zone_status() {
    local zone=$1
    local usage_pct=$2
    local remaining=$3
    local required=$4

    case "$zone" in
    "GREEN")
        echo -e "${GREEN}🟢 GREEN ZONE${NC} (${usage_pct}% usage)"
        echo -e "${GREEN}📊 Current: $CURRENT_COUNT/$MAX_MESSAGES messages${NC}"
        echo -e "${GREEN}📋 Required for $COMPLEXITY task: ~$required messages${NC}"
        echo -e "${GREEN}✅ Safe to continue development${NC}"
        ;;
    "YELLOW")
        echo -e "${YELLOW}🟡 YELLOW ZONE${NC} (${usage_pct}% usage)"
        echo -e "${YELLOW}⚠️  WARNING: Chat limit approaching!${NC}"
        echo -e "${YELLOW}📊 Remaining capacity: ~$remaining messages${NC}"
        echo -e "${YELLOW}📋 Required for $COMPLEXITY task: ~$required messages${NC}"
        if [ "$remaining" -lt "$required" ]; then
            echo -e "${YELLOW}🔄 RECOMMENDATION: Start preparing handoff${NC}"
            show_handoff_preparation
        else
            echo -e "${YELLOW}⏰ Monitor closely and prepare for transition${NC}"
        fi
        ;;
    "RED")
        echo -e "${RED}🔴 RED ZONE${NC} (${usage_pct}% usage)"
        echo -e "${RED}🚨 URGENT: Chat limit critically approaching!${NC}"
        echo -e "${RED}📊 Remaining capacity: ~$remaining messages${NC}"
        echo -e "${RED}📋 Required for $COMPLEXITY task: ~$required messages${NC}"
        echo -e "${RED}🔄 ACTION REQUIRED: Generate handoff immediately${NC}"
        show_immediate_handoff_instructions
        ;;
    "CRITICAL")
        echo -e "${RED}🚨 CRITICAL ZONE${NC} (${usage_pct}% usage)"
        echo -e "${RED}💥 IMMEDIATE ACTION: Chat limit exceeded!${NC}"
        echo -e "${RED}📊 Current: $CURRENT_COUNT/$MAX_MESSAGES (OVER LIMIT)${NC}"
        echo -e "${RED}🔄 EMERGENCY: Create new chat immediately${NC}"
        show_emergency_instructions
        ;;
    esac
}

# Function to show handoff preparation
show_handoff_preparation() {
    echo ""
    echo -e "${CYAN}📝 引き継ぎ準備ステップ:${NC}"
    echo "   1. 可能であれば現在のサブタスクを完了"
    echo "   2. コードが動作状態であることを確認"
    echo "   3. 実行: ./generate_handoff.sh [task] [progress%] [style] [file]"
    echo "   4. 新しいチャットへの移行準備"
}

# Function to show immediate handoff instructions
show_immediate_handoff_instructions() {
    echo ""
    echo -e "${CYAN}🔥 即座引き継ぎ指示:${NC}"
    echo "   1. 現在の開発を停止"
    echo "   2. 現在の動作コードを保存"
    echo "   3. 実行: ./generate_handoff.sh [task] [progress] [style] [code_file]"
    echo "   4. 引き継ぎテンプレートを新しいチャットにコピー"
    echo ""
    echo -e "${YELLOW}実行例:${NC}"
    echo "   ./generate_handoff.sh \"A1.3-ErrorHandling\" \"80%\" \"A+ToT\" \"src/error.rs\""
}

# Function to show emergency instructions
show_emergency_instructions() {
    echo ""
    echo -e "${RED}🚨 緊急プロトコル:${NC}"
    echo "   1. 即座に現在のタスクを停止"
    echo "   2. 動作するコードをファイルに保存"
    echo "   3. 最小限の引き継ぎメモを作成"
    echo "   4. 緊急テンプレートで新しいチャット開始"
    echo ""
    echo "🚨 緊急引き継ぎプロトコル:"
    echo "   Copy to new chat:"
    echo "---"
    echo "🚨 緊急継続"
    echo "前回チャットが制限に達しました。"
    echo "タスク: [タスク名]"
    echo "進捗: [%]"
    echo "最終動作コード: [コードスニペット]"
    echo "緊急課題: [次のアクション]"
    echo "スタイル: [A/B/C]"
    echo "言語: 日本語での回答をお願いします"
    echo "---"
}

# Function to provide optimization suggestions
provide_optimization_suggestions() {
    local zone=$1
    local complexity=$2

    echo ""
    echo -e "${CYAN}💡 最適化提案:${NC}"

    case "$zone" in
    "GREEN" | "YELLOW")
        echo "   • より焦点を絞ったプロンプトを使用"
        echo "   • 関連する質問をまとめて送信"
        echo "   • コードを完全なブロックで要求"
        if [[ "$complexity" == "ToT" ]]; then
            echo "   • 簡単なサブタスクは単一専門家に切り替え検討"
        fi
        ;;
    "RED" | "CRITICAL")
        echo "   • 必須機能のみに優先順位を付ける"
        echo "   • ドキュメント作成を次のチャットに延期"
        echo "   • 速度重視でStyle B（実装）に切り替え"
        echo "   • 複雑なタスクをより小さなチャンクに分割"
        ;;
    esac
}

# Main execution
main() {
    # Calculate metrics
    local usage_pct=$((CURRENT_COUNT * 100 / MAX_MESSAGES))
    local remaining=$((MAX_MESSAGES - CURRENT_COUNT))
    local required=$(get_required_messages "$COMPLEXITY")
    local zone=$(get_zone "$usage_pct")

    # Header
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}📊 ZYNAPSE CHAT USAGE MONITOR${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Display status
    display_zone_status "$zone" "$usage_pct" "$remaining" "$required"

    # Show additional info for non-green zones
    if [[ "$zone" != "GREEN" ]]; then
        provide_optimization_suggestions "$zone" "$COMPLEXITY"
    fi

    # Log to session if available
    local latest_session=$(ls -t ../sessions/session_*.log 2>/dev/null | head -1)
    if [ -n "$latest_session" ]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] USAGE CHECK: $CURRENT_COUNT/$MAX_MESSAGES ($usage_pct%) - $zone ZONE" >>"$latest_session"
    fi

    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Execute main function
main
