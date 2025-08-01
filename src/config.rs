//! Configuration management for Zynapse
//! Zynapseの設定管理
//!
//! This module handles loading, validating, and managing configuration settings
//! for the Zynapse application across different environments and use cases.
//! このモジュールは異なる環境と使用ケースにわたってZynapseアプリケーションの
//! 設定の読み込み、検証、管理を処理します。

use crate::{Result, ZynapseError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for Zynapse
/// Zynapseのメイン設定構造体
///
/// This structure contains all configuration options for Zynapse,
/// organized by functional areas.
/// この構造体は機能領域別に整理されたZynapseのすべての設定オプションを含みます。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Storage configuration
    /// ストレージ設定
    pub storage: StorageConfig,

    /// Search configuration
    /// 検索設定
    #[cfg(feature = "search")]
    pub search: SearchConfig,

    /// CLI configuration
    /// CLI設定
    #[cfg(feature = "cli")]
    pub cli: CliConfig,

    /// TUI configuration
    /// TUI設定
    #[cfg(feature = "tui")]
    pub tui: TuiConfig,

    /// Logging configuration
    /// ログ設定
    pub logging: LoggingConfig,
}

/// Storage-related configuration
/// ストレージ関連設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Root directory for storing notes
    /// ノート保存用ルートディレクトリ
    pub root_path: PathBuf,

    /// Maximum file size in bytes (default: 10MB)
    /// 最大ファイルサイズ（バイト単位、デフォルト：10MB）
    pub max_file_size: u64,

    /// Backup configuration
    /// バックアップ設定
    pub backup: BackupConfig,

    /// Auto-save interval in seconds (0 = disabled)
    /// 自動保存間隔（秒単位、0 = 無効）
    pub auto_save_interval: u64,
}

/// Backup configuration
/// バックアップ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    /// 自動バックアップを有効にする
    pub enabled: bool,

    /// Backup directory path
    /// バックアップディレクトリパス
    pub path: PathBuf,

    /// Number of backups to retain
    /// 保持するバックアップ数
    pub retain_count: u32,
}

/// Search engine configuration
/// 検索エンジン設定
#[cfg(feature = "search")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Index directory path
    /// インデックスディレクトリパス
    pub index_path: PathBuf,

    /// Maximum search results to return
    /// 返す最大検索結果数
    pub max_results: usize,

    /// Enable fuzzy search
    /// ファジー検索を有効にする
    pub fuzzy_search: bool,

    /// Search timeout in milliseconds
    /// 検索タイムアウト（ミリ秒）
    pub timeout_ms: u64,
}

/// CLI-specific configuration
/// CLI固有設定
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Default editor command
    /// デフォルトエディタコマンド
    pub editor: String,

    /// Enable colored output
    /// カラー出力を有効にする
    pub colored_output: bool,

    /// Maximum items to display in lists
    /// リストで表示する最大項目数
    pub max_list_items: usize,
}

/// TUI-specific configuration
/// TUI固有設定
#[cfg(feature = "tui")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Theme name
    /// テーマ名
    pub theme: String,

    /// Frame rate (FPS) for TUI updates
    /// TUI更新用フレームレート（FPS）
    pub frame_rate: u32,

    /// Enable mouse support
    /// マウスサポートを有効にする
    pub mouse_support: bool,

    /// Key bindings configuration
    /// キーバインド設定
    pub keybindings: KeyBindings,
}

/// Key binding configuration for TUI
/// TUI用キーバインド設定
#[cfg(feature = "tui")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    /// Key to quit the application
    /// アプリケーション終了キー
    pub quit: String,

    /// Key to search
    /// 検索キー
    pub search: String,

    /// Key to create new note
    /// 新規ノート作成キー
    pub new_note: String,

    /// Key to edit current note
    /// 現在のノート編集キー
    pub edit: String,
}

/// Logging configuration
/// ログ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    /// ログレベル（error, warn, info, debug, trace）
    pub level: String,

    /// Log file path (None = stdout only)
    /// ログファイルパス（None = 標準出力のみ）
    pub file_path: Option<PathBuf>,

    /// Enable timestamp in logs
    /// ログにタイムスタンプを有効にする
    pub timestamp: bool,

    /// Enable colored logs (for terminal output)
    /// カラーログを有効にする（ターミナル出力用）
    pub colored: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage: StorageConfig::default(),

            #[cfg(feature = "search")]
            search: SearchConfig::default(),

            #[cfg(feature = "cli")]
            cli: CliConfig::default(),

            #[cfg(feature = "tui")]
            tui: TuiConfig::default(),

            logging: LoggingConfig::default(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            root_path: home_dir.join(".zynapse").join("notes"),
            max_file_size: 10 * 1024 * 1024, // 10MB
            backup: BackupConfig::default(),
            auto_save_interval: 300, // 5 minutes
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            enabled: true,
            path: home_dir.join(".zynapse").join("backups"),
            retain_count: 10,
        }
    }
}

#[cfg(feature = "search")]
impl Default for SearchConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            index_path: home_dir.join(".zynapse").join("index"),
            max_results: 100,
            fuzzy_search: true,
            timeout_ms: 5000, // 5 seconds
        }
    }
}

#[cfg(feature = "cli")]
impl Default for CliConfig {
    fn default() -> Self {
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| {
                if cfg!(windows) {
                    "notepad".to_string()
                } else {
                    "nano".to_string()
                }
            });

        Self {
            editor,
            colored_output: true,
            max_list_items: 50,
        }
    }
}

#[cfg(feature = "tui")]
impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            frame_rate: 60,
            mouse_support: true,
            keybindings: KeyBindings::default(),
        }
    }
}

#[cfg(feature = "tui")]
impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: "q".to_string(),
            search: "/".to_string(),
            new_note: "n".to_string(),
            edit: "e".to_string(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: None,
            timestamp: true,
            colored: true,
        }
    }
}

impl Config {
    /// Load configuration from the default config file
    /// デフォルト設定ファイルから設定を読み込み
    ///
    /// Loads configuration from `~/.zynapse/config.toml` or creates a default
    /// configuration if the file doesn't exist.
    /// `~/.zynapse/config.toml`から設定を読み込み、ファイルが存在しない場合は
    /// デフォルト設定を作成します。
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// 以下の場合にエラーを返します：
    /// - Configuration file exists but cannot be read
    /// - Configuration file contains invalid TOML
    /// - Required directories cannot be created
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zynapse::config::Config;
    ///
    /// let config = Config::load()?;
    /// println!("Notes directory: {:?}", config.storage.root_path);
    /// # Ok::<(), zynapse::ZynapseError>(())
    /// ```
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Load configuration from a specific file
    /// 特定のファイルから設定を読み込み
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `path` - Path to the configuration file / 設定ファイルへのパス
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    /// ファイルが読み取れないまたは解析できない場合にエラーを返します。
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            ZynapseError::io_error(e, format!("Failed to read config file: {:?}", path))
        })?;

        let config: Self = toml::from_str(&content).map_err(|e| {
            ZynapseError::config_error(format!("Invalid TOML in config file: {}", e))
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to the default config file
    /// デフォルト設定ファイルに設定を保存
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be serialized or written.
    /// 設定がシリアライズまたは書き込みできない場合にエラーを返します。
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        self.save_to_file(&config_path)
    }

    /// Save configuration to a specific file
    /// 特定のファイルに設定を保存
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `path` - Path where to save the configuration / 設定を保存するパス
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        // 親ディレクトリが存在しない場合は作成
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ZynapseError::io_error(e, "Failed to create config directory"))?;
        }

        let content = toml::to_string(self).map_err(|e| {
            ZynapseError::config_error(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(path, content).map_err(|e| {
            ZynapseError::io_error(e, format!("Failed to write config file: {:?}", path))
        })?;

        Ok(())
    }

    /// Validate the configuration
    /// 設定を検証
    ///
    /// Checks that all configuration values are valid and consistent.
    /// すべての設定値が有効で一貫していることをチェックします。
    pub fn validate(&self) -> Result<()> {
        // Validate storage configuration
        // ストレージ設定を検証
        if self.storage.max_file_size == 0 {
            return Err(ZynapseError::config_error(
                "max_file_size must be greater than 0",
            ));
        }

        if self.storage.backup.retain_count == 0 {
            return Err(ZynapseError::config_error(
                "backup.retain_count must be greater than 0",
            ));
        }

        // Validate search configuration
        // 検索設定を検証
        #[cfg(feature = "search")]
        {
            if self.search.max_results == 0 {
                return Err(ZynapseError::config_error(
                    "search.max_results must be greater than 0",
                ));
            }

            if self.search.timeout_ms == 0 {
                return Err(ZynapseError::config_error(
                    "search.timeout_ms must be greater than 0",
                ));
            }
        }

        // Validate CLI configuration
        // CLI設定を検証
        #[cfg(feature = "cli")]
        {
            if self.cli.editor.is_empty() {
                return Err(ZynapseError::config_error("cli.editor cannot be empty"));
            }

            if self.cli.max_list_items == 0 {
                return Err(ZynapseError::config_error(
                    "cli.max_list_items must be greater than 0",
                ));
            }
        }

        // Validate TUI configuration
        // TUI設定を検証
        #[cfg(feature = "tui")]
        {
            if self.tui.frame_rate == 0 || self.tui.frame_rate > 120 {
                return Err(ZynapseError::config_error(
                    "tui.frame_rate must be between 1 and 120",
                ));
            }
        }

        // Validate logging configuration
        // ログ設定を検証
        match self.logging.level.as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => {}
            _ => {
                return Err(ZynapseError::config_error(
                    "logging.level must be one of: error, warn, info, debug, trace",
                ))
            }
        }

        Ok(())
    }

    /// Get the default configuration file path
    /// デフォルト設定ファイルパスを取得
    fn config_file_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ZynapseError::config_error("Cannot determine home directory"))?;

        Ok(home_dir.join(".zynapse").join("config.toml"))
    }

    /// Create all necessary directories based on the configuration
    /// 設定に基づいて必要なすべてのディレクトリを作成
    ///
    /// # Errors
    ///
    /// Returns an error if any directory cannot be created.
    /// いずれかのディレクトリが作成できない場合にエラーを返します。
    pub fn create_directories(&self) -> Result<()> {
        // Create storage directory
        // ストレージディレクトリを作成
        std::fs::create_dir_all(&self.storage.root_path)
            .map_err(|e| ZynapseError::io_error(e, "Failed to create storage directory"))?;

        // Create backup directory if backups are enabled
        // バックアップが有効な場合はバックアップディレクトリを作成
        if self.storage.backup.enabled {
            std::fs::create_dir_all(&self.storage.backup.path)
                .map_err(|e| ZynapseError::io_error(e, "Failed to create backup directory"))?;
        }

        // Create search index directory
        // 検索インデックスディレクトリを作成
        #[cfg(feature = "search")]
        {
            std::fs::create_dir_all(&self.search.index_path).map_err(|e| {
                ZynapseError::io_error(e, "Failed to create search index directory")
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[storage]"));
        assert!(toml_str.contains("[logging]"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_content = r#"
[storage]
root_path = "/tmp/zynapse/notes"
max_file_size = 5242880
auto_save_interval = 60

[storage.backup]
enabled = true
path = "/tmp/zynapse/backups"
retain_count = 5

[logging]
level = "debug"
timestamp = true
colored = false
"#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.storage.max_file_size, 5242880);
        assert_eq!(config.storage.auto_save_interval, 60);
        assert_eq!(config.logging.level, "debug");
        assert!(!config.logging.colored);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_validation() {
        let mut config = Config::default();

        // Test invalid max_file_size
        config.storage.max_file_size = 0;
        assert!(config.validate().is_err());

        // Reset and test invalid log level
        config = Config::default();
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let original_config = Config::default();
        original_config.save_to_file(&config_path).unwrap();

        let loaded_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(
            original_config.storage.max_file_size,
            loaded_config.storage.max_file_size
        );
        assert_eq!(original_config.logging.level, loaded_config.logging.level);
    }

    #[cfg(feature = "cli")]
    #[test]
    fn test_cli_config_defaults() {
        let config = Config::default();
        assert!(!config.cli.editor.is_empty());
        assert!(config.cli.colored_output);
        assert!(config.cli.max_list_items > 0);
    }
}
