# Zynapse Chat Management Scripts

このディレクトリには、Zynapse開発中のAIチャットセッション管理用bashスクリプトが含まれています。

## 🚀 クイックスタート

```bash
# 1. 日次開発開始
./daily_development.sh

# 2. 開発中のチャット使用量監視
./chat_usage_monitor.sh 45 "Medium"

# 3. 制限接近時の引き継ぎ生成
./generate_handoff.sh "A1.3-ErrorHandling" "75%" "A+ToT" "src/error.rs"

# 4. セッション終了
./end_session.sh "completed" "エラー型の実装に成功しました"
```

## 📁 スクリプト概要

### `daily_development.sh`

**目的**: インタラクティブなタスク選択で日次開発を開始
**使用法**: `./daily_development.sh [WEEK] [TASK]`
**機能**:

- gitブランチからの現在タスク自動検出
- 開発ダッシュボード表示
- 最適な開発スタイル提案
- 開発環境セットアップ

### `zynapse_session_start.sh`

**目的**: AIチャットセッションテンプレート生成
**使用法**: `./zynapse_session_start.sh WEEK TASK STYLE TOT_MODE`
**例**: `./zynapse_session_start.sh "Week1" "A1.3-ErrorHandling" "A+ToT" "ToT"`
**機能**:

- 構造化チャットテンプレート生成
- メッセージ要件推定
- ToT専門家チーム提案
- 環境検証

### `chat_usage_monitor.sh`

**目的**: チャット使用量監視と制限オーバーラン防止
**使用法**: `./chat_usage_monitor.sh CURRENT_COUNT [COMPLEXITY]`
**例**: `./chat_usage_monitor.sh 65 "High"`
**機能**:

- 色分けゾーン警告（Green/Yellow/Red/Critical）
- 残り容量推定
- 最適化提案提供
- 引き継ぎ準備トリガー

### `generate_handoff.sh`

**目的**: 包括的引き継ぎドキュメント作成
**使用法**: `./generate_handoff.sh TASK_NAME PROGRESS STYLE CODE_FILE`
**例**: `./generate_handoff.sh "A1.3-ErrorHandling" "80%" "A+ToT" "src/error.rs"`
**機能**:

- 詳細markdownドキュメント生成
- コード状態とプロジェクト状況記録
- 即座再開テンプレート提供
- 設計決定とコンテキスト追跡

### `end_session.sh`

**目的**: 開発セッション適切終了
**使用法**: `./end_session.sh [OUTCOME] [NOTES]`
**例**: `./end_session.sh "completed" "エラー型が完全実装されました"`
**機能**:

- セッションログ更新
- 品質チェック実行
- セッションサマリー提供
- 次アクション提案

## ⚙️ 設定

`chat_config.conf` を編集してカスタマイズ:

- メッセージ制限と閾値
- 複雑度推定
- カラースキーム
- 開発スタイルパラメータ

## 📁 ディレクトリ構造

```
scripts/
├── chat_config.conf              # 設定ファイル
├── zynapse_session_start.sh      # セッション開始
├── chat_usage_monitor.sh         # 使用量監視
├── generate_handoff.sh           # 引き継ぎ生成
├── daily_development.sh          # 日次ワークフロー
├── end_session.sh               # セッション終了
└── README.md                    # このファイル

../sessions/                     # セッションログ
../handoffs/                     # 引き継ぎドキュメント
../learning_log.md              # 学習進捗
```

## 🔄 ワークフロー例

### 通常の開発セッション

```bash
# 1日の開始
./daily_development.sh

# 開発中（20-30メッセージごと）
./chat_usage_monitor.sh 25 "Medium"
./chat_usage_monitor.sh 55 "Medium"  # Yellow zone警告

# 制限接近
./chat_usage_monitor.sh 75 "Medium"  # Red zone - 引き継ぎ準備
./generate_handoff.sh "A1.3-ErrorHandling" "85%" "A+ToT" "src/error.rs"

# セッション終了
./end_session.sh "completed" "タスクが正常に完了しました"
```

### 緊急引き継ぎ

```bash
# Critical zone到達
./chat_usage_monitor.sh 95 "High"   # 緊急警告
./generate_handoff.sh "F1.1-Search" "60%" "C+ToT" "src/search.rs"

# 生成されたテンプレートで新チャット即座開始
# 新セッションで継続
```

### 複数日タスク

```bash
# 1日目
./daily_development.sh "Week2" "B1.1-NoteStructure"
# ... 開発作業 ...
./generate_handoff.sh "B1.1-NoteStructure" "45%" "C+ToT" "src/note.rs"
./end_session.sh "handoff" "1日目完了、構造定義済み"

# 2日目
./daily_development.sh "Week2" "B1.1-NoteStructure-Continue"
# 引き継ぎテンプレートで再開
# ... 開発継続 ...
./end_session.sh "completed" "構造実装完了"
```

## 🛠️ トラブルシューティング

### よくある問題

1. **Permission denied**: `chmod +x *.sh` を実行
2. **Config not found**: `chat_config.conf` が存在することを確認
3. **No git repository**: `git init` で初期化
4. **Cargo.toml missing**: `cargo init --name zynapse` を実行

### 復旧手順

- **セッション喪失**: `../sessions/` で最近のログを確認
- **引き継ぎ不明**: `./generate_handoff.sh` で再生成
- **環境破損**: `./daily_development.sh` でセットアップ再実行

## 📈 ベストプラクティス

1. **毎日 `daily_development.sh` で開始**
2. **20-30メッセージごとに使用量監視**
3. **Yellow zone（60%+）で引き継ぎ生成**
4. **`end_session.sh` でセッション適切終了**
5. **継続前に引き継ぎドキュメント確認**
6. **ワークフローに合わせて `chat_config.conf` 調整**

## 🎯 Zynapse開発との統合

これらのスクリプトはZynapse Phase 1開発ワークフロー専用設計:

- **Week 1-2**: 基盤タスク（A1.x, A2.x シリーズ）
- **Week 3-4**: CLIと基本機能（D1.x, E1.x シリーズ）
- **Week 5-6**: 複雑機能（F1.x, G1.x シリーズ）
- **Week 7-8**: 統合と仕上げ（H1.x, H2.x, H3.x シリーズ）

各スクリプトはZynapseタスク命名を理解し、開発スタイルとToT専門家チームの適切な提案を提供します。
