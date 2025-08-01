#!/bin/bash
# Daily Development Workflow Automation
# Usage: ./daily_development.sh [WEEK] [TASK]

set -e

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/chat_config.conf"

# Parameters with smart defaults
WEEK=${1:-"Week1"}
TASK=${2:-""}
AUTO_DETECT=${3:-"yes"}

# Function to detect current task from git branch or directory
detect_current_task() {
    local detected_task=""

    # Try to detect from git branch
    if git rev-parse --git-dir >/dev/null 2>&1; then
        local branch=$(git branch --show-current 2>/dev/null)
        if [[ "$branch" =~ (A[0-9]+\.[0-9]+|B[0-9]+\.[0-9]+|C[0-9]+\.[0-9]+|D[0-9]+\.[0-9]+|E[0-9]+\.[0-9]+|F[0-9]+\.[0-9]+|G[0-9]+\.[0-9]+|H[0-9]+\.[0-9]+) ]]; then
            detected_task="$branch"
        fi
    fi

    # Try to detect from recent session logs
    if [ -z "$detected_task" ] && [ -d "../sessions" ]; then
        local latest_session=$(ls -t ../sessions/session_*.log 2>/dev/null | head -1)
        if [ -n "$latest_session" ]; then
            detected_task=$(grep "Task:" "$latest_session" | tail -1 | cut -d' ' -f2-)
        fi
    fi

    echo "$detected_task"
}

# Function to suggest style based on task
suggest_style_for_task() {
    local task="$1"
    local week_num=$(echo "$WEEK" | grep -o '[0-9]*' || echo "1")

    case "$task" in
    *"Error"* | *"error"* | A1.*)
        echo "A+ToT" # Learning-focused for foundational work
        ;;
    *"CLI"* | D1.*)
        echo "B" # Implementation-focused for CLI
        ;;
    *"Search"* | *"Tantivy"* | F1.*)
        echo "C+ToT" # Pair programming for complex integrations
        ;;
    *"TUI"* | G1.*)
        echo "C+ToT" # Pair programming for UI work
        ;;
    *"Note"* | *"Data"* | B1.*)
        echo "C+ToT" # Collaborative design for data models
        ;;
    *)
        # Default based on week
        if [ "$week_num" -le 2 ]; then
            echo "A+ToT" # Learning focus early
        elif [ "$week_num" -le 4 ]; then
            echo "C" # Balanced approach mid-project
        else
            echo "B" # Speed focus later
        fi
        ;;
    esac
}

# Function to show development dashboard
show_dashboard() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}📊 ZYNAPSE 開発ダッシュボード${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Project status
    echo -e "${PURPLE}📋 プロジェクトステータス${NC}"
    if [ -f "../Cargo.toml" ]; then
        cd ..
        echo "   プロジェクト: $(grep '^name' Cargo.toml | cut -d'"' -f2) $(grep '^version' Cargo.toml | cut -d'"' -f2)"
        echo "   コンパイル状況: $(cargo check --quiet 2>&1 && echo -e "${GREEN}✅ OK${NC}" || echo -e "${RED}❌ 問題あり${NC}")"
        echo "   テスト状況: $(cargo test --quiet 2>&1 && echo -e "${GREEN}✅ Pass${NC}" || echo -e "${YELLOW}⚠️  一部失敗${NC}")"
        cd scripts
    fi

    # Git status
    if git rev-parse --git-dir >/dev/null 2>&1; then
        echo "   Gitブランチ: $(git branch --show-current)"
        local changes=$(git status --porcelain | wc -l)
        echo "   未コミット変更: $changes"
    fi

    echo ""

    # Recent sessions
    echo -e "${PURPLE}📈 最近のセッション${NC}"
    if [ -d "../sessions" ]; then
        local session_count=$(ls -1 ../sessions/session_*.log 2>/dev/null | wc -l)
        echo "   総セッション数: $session_count"

        if [ "$session_count" -gt 0 ]; then
            echo "   最近のセッション:"
            ls -t ../sessions/session_*.log 2>/dev/null | head -3 | while read session; do
                local date=$(basename "$session" | cut -d'_' -f2)
                local task=$(grep "Task:" "$session" | head -1 | cut -d' ' -f2- | cut -c1-30)
                echo "     • $date: $task..."
            done
        fi
    else
        echo "   前回のセッションが見つかりません"
    fi

    echo ""

    # Recent handoffs
    echo -e "${PURPLE}🔄 最近の引き継ぎ${NC}"
    if [ -d "../handoffs" ]; then
        local handoff_count=$(ls -1 ../handoffs/handoff_*.md 2>/dev/null | wc -l)
        echo "   総引き継ぎ数: $handoff_count"

        if [ "$handoff_count" -gt 0 ]; then
            echo "   最近の引き継ぎ:"
            ls -t ../handoffs/handoff_*.md 2>/dev/null | head -3 | while read handoff; do
                local date=$(basename "$handoff" | cut -d'_' -f2)
                local task=$(basename "$handoff" | cut -d'_' -f3- | sed 's/.md$//' | cut -c1-25)
                echo "     • $date: $task..."
            done
        fi
    else
        echo "   前回の引き継ぎが見つかりません"
    fi

    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Interactive task selection
interactive_task_selection() {
    echo -e "${CYAN}🎯 タスク選択${NC}"

    # Show available tasks based on week
    echo "$WEEK の利用可能タスク:"
    case "$WEEK" in
    *"1"*)
        echo "   A1.1 - Cargo.toml セットアップ"
        echo "   A1.2 - lib.rs 構造"
        echo "   A1.3 - エラー型定義"
        echo "   A1.4 - 品質チェックスクリプト"
        echo "   A1.5 - 設定セットアップ"
        ;;
    *"2"*)
        echo "   B1.1 - Note構造体定義"
        echo "   B1.2 - NoteFrontmatter構造体"
        echo "   B1.6 - ファイル名生成ロジック"
        echo "   C1.2 - create_note実装"
        ;;
    *)
        echo "   カスタムタスク - 手動で指定してください"
        ;;
    esac

    echo ""
    read -p "タスクIDを入力 (自動検出の場合はEnter): " selected_task

    if [ -z "$selected_task" ]; then
        selected_task=$(detect_current_task)
        if [ -n "$selected_task" ]; then
            echo "   🔍 自動検出: $selected_task"
        else
            echo "   ❓ タスクを自動検出できませんでした。手動で指定してください。"
            read -p "タスクID: " selected_task
        fi
    fi

    echo "$selected_task"
}

# Main execution flow
main() {
    echo -e "${CYAN}🌅 おはようございます！Zynapse開発を開始します...${NC}"
    echo ""

    # Show dashboard
    show_dashboard

    # Determine task
    if [ -z "$TASK" ] && [ "$AUTO_DETECT" = "yes" ]; then
        TASK=$(interactive_task_selection)
    elif [ -z "$TASK" ]; then
        TASK=$(detect_current_task)
    fi

    if [ -z "$TASK" ]; then
        echo -e "${RED}❌ タスクが指定されていないか検出できませんでした${NC}"
        echo "使用法: $0 [WEEK] [TASK] [AUTO_DETECT]"
        exit 1
    fi

    # Suggest style
    SUGGESTED_STYLE=$(suggest_style_for_task "$TASK")
    echo ""
    echo -e "${YELLOW}💡 $TASK の推奨開発スタイル: $SUGGESTED_STYLE${NC}"
    read -p "推奨スタイルを使用しますか？ (Y/n): " use_suggested

    if [[ "$use_suggested" =~ ^[Nn] ]]; then
        echo "利用可能なスタイル:"
        echo "   A - レビューモード (学習重視)"
        echo "   B - 実装モード (速度重視)"
        echo "   C - ペアプログラミングモード (バランス)"
        echo "   +ToT を追加でTree of Thoughtsモード"
        read -p "スタイルを入力: " STYLE
    else
        STYLE="$SUGGESTED_STYLE"
    fi

    # Determine ToT mode
    TOT_MODE="No"
    if [[ "$STYLE" == *"ToT"* ]]; then
        TOT_MODE="ToT"
    fi

    echo ""
    echo -e "${GREEN}🚀 開発セッションを開始します...${NC}"
    echo "   週: $WEEK"
    echo "   タスク: $TASK"
    echo "   スタイル: $STYLE"
    echo "   ToT: $TOT_MODE"
    echo ""

    # Start the session
    ./zynapse_session_start.sh "$WEEK" "$TASK" "$STYLE" "$TOT_MODE"

    echo ""
    echo -e "${YELLOW}📝 開発時のリマインダー:${NC}"
    echo "   • チャット使用量監視: ./chat_usage_monitor.sh [count] [complexity]"
    echo "   • 必要時に引き継ぎ生成: ./generate_handoff.sh"
    echo "   • セッション終了: ./end_session.sh"
    echo ""
    echo -e "${CYAN}🎯 Happy coding! 品質第一、速度は二の次です。${NC}"
}

# Execute main function
main "$@"
