#!/bin/bash
# End Development Session and Update Logs
# Usage: ./end_session.sh [SESSION_OUTCOME] [NOTES]

set -e

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/chat_config.conf"

# Parameters
SESSION_OUTCOME=${1:-"completed"} # completed, handoff, interrupted
NOTES=${2:-""}

# Timestamp
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Function to find latest session
find_latest_session() {
    ls -t ../sessions/session_*.log 2>/dev/null | head -1
}

# Function to run quality check
run_quality_check() {
    if [ -f "../Cargo.toml" ]; then
        cd ..
        local fmt_status=$(cargo fmt --check 2>&1 && echo "✅" || echo "❌")
        local clippy_status=$(cargo clippy --quiet 2>&1 && echo "✅" || echo "❌")
        local test_status=$(cargo test --quiet 2>&1 && echo "✅" || echo "❌")
        cd scripts
        echo "fmt:$fmt_status clippy:$clippy_status test:$test_status"
    else
        echo "cargoプロジェクトなし"
    fi
}

# Function to update session log
update_session_log() {
    local session_file="$1"
    local outcome="$2"
    local notes="$3"

    cat >>"$session_file" <<LOG

[$TIMESTAMP] SESSION END
Outcome: $outcome
Notes: $notes
Quality Check: $(run_quality_check)
LOG
}

# Function to generate session summary
generate_session_summary() {
    local session_file="$1"

    echo -e "${CYAN}📊 セッションサマリー${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    if [ -f "$session_file" ]; then
        local start_time=$(grep "SESSION START" "$session_file" | head -1 | cut -d']' -f1 | cut -d'[' -f2)
        local task=$(grep "Task:" "$session_file" | head -1 | cut -d' ' -f2-)
        local style=$(grep "Style:" "$session_file" | head -1 | cut -d' ' -f2-)

        echo "   📋 タスク: $task"
        echo "   🎨 スタイル: $style"
        echo "   🕐 開始: $start_time"
        echo "   🕑 終了: $TIMESTAMP"
        echo "   🎯 結果: $SESSION_OUTCOME"

        # Show usage counts if available
        local usage_checks=$(grep "USAGE CHECK" "$session_file" | wc -l)
        if [ "$usage_checks" -gt 0 ]; then
            local final_usage=$(grep "USAGE CHECK" "$session_file" | tail -1 | grep -o '[0-9]*\/[0-9]*')
            echo "   📊 チャット使用量: $final_usage (${usage_checks}回チェック)"
        fi

        # Show handoffs if any
        local handoffs=$(grep "HANDOFF GENERATED" "$session_file" | wc -l)
        if [ "$handoffs" -gt 0 ]; then
            echo "   🔄 引き継ぎ: ${handoffs}件生成"
        fi
    fi

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Function to update learning log
update_learning_log() {
    local learning_log="../learning_log.md"

    echo "" >>"$learning_log"
    echo "## $(date '+%Y-%m-%d'): セッション終了" >>"$learning_log"
    echo "- **結果**: $SESSION_OUTCOME" >>"$learning_log"
    echo "- **品質チェック**: $(run_quality_check)" >>"$learning_log"

    if [ -n "$NOTES" ]; then
        echo "- **メモ**: $NOTES" >>"$learning_log"
    fi

    echo "- **セッションファイル**: $(find_latest_session | xargs basename)" >>"$learning_log"
}

# Function to suggest next actions
suggest_next_actions() {
    echo -e "${YELLOW}💡 推奨次アクション:${NC}"

    case "$SESSION_OUTCOME" in
    "completed")
        echo "   • セッションの学習ポイントを確認"
        echo "   • 次の開発セッションを計画"
        echo "   • プロジェクトドキュメントを更新"
        echo "   • セッション変更でgitコミットを検討"
        ;;
    "handoff")
        echo "   • 生成された引き継ぎドキュメントを確認"
        echo "   • 新しいチャットセッションの準備"
        echo "   • コードが動作状態であることを確認"
        echo "   • 準備ができたら新セッション開始"
        ;;
    "interrupted")
        echo "   • 現在の作業状態を保存"
        echo "   • 次セッション用の停止ポイントをメモ"
        echo "   • クイック引き継ぎの作成を検討"
        echo "   • 再開戦略を計画"
        ;;
    *)
        echo "   • セッション結果を確認"
        echo "   • 継続戦略を計画"
        ;;
    esac
}

# Function to show quality recommendations
show_quality_recommendations() {
    echo -e "${PURPLE}🔍 品質推奨事項:${NC}"

    if [ -f "../Cargo.toml" ]; then
        cd ..

        # Check formatting
        if ! cargo fmt --check >/dev/null 2>&1; then
            echo "   📝 実行: cargo fmt"
        fi

        # Check clippy
        if ! cargo clippy --quiet >/dev/null 2>&1; then
            echo "   🔍 実行: cargo clippy"
        fi

        # Check tests
        if ! cargo test --quiet >/dev/null 2>&1; then
            echo "   🧪 失敗テストを修正: cargo test"
        fi

        # Check documentation
        if ! cargo doc --no-deps >/dev/null 2>&1; then
            echo "   📚 ドキュメント修正: cargo doc"
        fi

        cd scripts
    fi
}

# Main execution
main() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}🏁 ZYNAPSE 開発セッション終了${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Find and update latest session
    local latest_session=$(find_latest_session)

    if [ -n "$latest_session" ]; then
        update_session_log "$latest_session" "$SESSION_OUTCOME" "$NOTES"
        generate_session_summary "$latest_session"
    else
        echo -e "${YELLOW}⚠️  アクティブなセッションが見つかりませんでした${NC}"
    fi

    # Update learning log
    update_learning_log

    echo ""

    # Show quality status
    show_quality_recommendations

    echo ""

    # Suggest next actions
    suggest_next_actions

    echo ""
    echo -e "${GREEN}✅ セッションが正常に終了しました${NC}"
    echo -e "${CYAN}🎯 Zynapseへの貢献ありがとうございました！${NC}"
}

# Execute main function
main "$@"
