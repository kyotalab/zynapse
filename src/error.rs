//! Error types and handling for Zynapse
//! Zynapseのエラー型とハンドリング
//!
//! This module provides comprehensive error handling with structured error types
//! that support both user-friendly messages and detailed debugging information.
//! このモジュールは、ユーザーフレンドリーなメッセージと詳細なデバッグ情報の
//! 両方をサポートする構造化エラー型による包括的エラーハンドリングを提供します。

use thiserror::Error;

/// The main error type for Zynapse operations
/// Zynapse操作のメインエラー型
///
/// This enum covers all possible error conditions that can occur during
/// Zynapse operations, from file system issues to search engine problems.
/// このenumはファイルシステムの問題から検索エンジンの問題まで、
/// Zynapse操作中に発生する可能性のあるすべてのエラー条件をカバーします。
#[derive(Error, Debug)]
pub enum ZynapseError {
    /// I/O operation failed
    /// I/O操作の失敗
    #[error("I/O operation failed: {message}")]
    Io {
        /// The underlying I/O error
        /// 基礎となるI/Oエラー
        #[source]
        source: std::io::Error,
        /// Additional context message
        /// 追加のコンテキストメッセージ
        message: String,
    },

    /// Configuration error
    /// 設定エラー
    #[error("Configuration error: {message}")]
    Configuration {
        /// Error description
        /// エラー説明
        message: String,
    },

    /// Note not found
    /// ノートが見つからない
    #[error("Note not found: {id}")]
    NoteNotFound {
        /// The note identifier that was not found
        /// 見つからなかったノートの識別子
        id: String,
    },

    /// Invalid note content
    /// 無効なノート内容
    #[error("Invalid note content: {reason}")]
    InvalidContent {
        /// Reason for invalidity
        /// 無効である理由
        reason: String,
    },

    /// Search engine error
    /// 検索エンジンエラー
    #[cfg(feature = "search")]
    #[error("Search engine error: {message}")]
    Search {
        /// Error message from search engine
        /// 検索エンジンからのエラーメッセージ
        message: String,
    },

    /// Storage operation failed
    /// ストレージ操作の失敗
    #[cfg(feature = "basic-storage")]
    #[error("Storage operation failed: {operation}")]
    Storage {
        /// The operation that failed
        /// 失敗した操作
        operation: String,
        /// The underlying error
        /// 基礎となるエラー
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Serialization/deserialization error
    /// シリアライゼーション/デシリアライゼーションエラー
    #[error("Serialization error: {message}")]
    Serialization {
        /// Error message
        /// エラーメッセージ
        message: String,
        /// The underlying serde error
        /// 基礎となるserdeエラー
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// CLI argument parsing error
    /// CLI引数解析エラー
    #[cfg(feature = "cli")]
    #[error("CLI error: {message}")]
    Cli {
        /// Error message
        /// エラーメッセージ
        message: String,
    },

    /// TUI interface error
    /// TUIインターフェースエラー
    #[cfg(feature = "tui")]
    #[error("TUI error: {message}")]
    Tui {
        /// Error message
        /// エラーメッセージ
        message: String,
    },

    /// Generic error for unexpected conditions
    /// 予期しない条件での汎用エラー
    #[error("Internal error: {message}")]
    Internal {
        /// Error description
        /// エラー説明
        message: String,
    },
}

/// Specialized Result type for Zynapse operations
/// Zynapse操作用の特化されたResult型
///
/// This type alias provides a convenient way to work with Results that
/// can return ZynapseError. It's used throughout the codebase.
/// この型エイリアスはZynapseErrorを返す可能性のあるResultを
/// 扱う便利な方法を提供します。コードベース全体で使用されます。
///
/// # Examples
///
/// ```rust
/// use zynapse::{Result, ZynapseError};
///
/// fn example_operation() -> Result<String> {
///     Ok("Success".to_string())
/// }
///
/// fn failing_operation() -> Result<()> {
///     Err(ZynapseError::Internal {
///         message: "Something went wrong".to_string(),
///     })
/// }
/// ```
pub type Result<T> = std::result::Result<T, ZynapseError>;

impl ZynapseError {
    /// Create an I/O error with additional context
    /// 追加コンテキスト付きのI/Oエラーを作成
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `error` - The underlying I/O error / 基礎となるI/Oエラー
    /// * `message` - Additional context message / 追加のコンテキストメッセージ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    /// let zynapse_error = ZynapseError::io_error(io_error, "Failed to read configuration");
    /// ```
    pub fn io_error(source: std::io::Error, message: impl Into<String>) -> Self {
        Self::Io {
            source,
            message: message.into(),
        }
    }

    /// Create a configuration error
    /// 設定エラーを作成
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `message` - Error description / エラー説明
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::config_error("Invalid YAML syntax in config file");
    /// ```
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a note not found error
    /// ノートが見つからないエラーを作成
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `id` - The note identifier / ノート識別子
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::note_not_found("note-123");
    /// ```
    pub fn note_not_found(id: impl Into<String>) -> Self {
        Self::NoteNotFound { id: id.into() }
    }

    /// Create an invalid content error
    /// 無効なコンテンツエラーを作成
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `reason` - Reason for invalidity / 無効である理由
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::invalid_content("Note content is empty");
    /// ```
    pub fn invalid_content(reason: impl Into<String>) -> Self {
        Self::InvalidContent {
            reason: reason.into(),
        }
    }

    /// Create an internal error
    /// 内部エラーを作成
    ///
    /// # Arguments
    /// # 引数
    ///
    /// * `message` - Error description / エラー説明
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::internal("Unexpected state reached");
    /// ```
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Check if this error is recoverable
    /// このエラーが回復可能かどうかをチェック
    ///
    /// Returns true if the operation can potentially be retried.
    /// 操作が潜在的に再試行可能な場合にtrueを返します。
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::config_error("Invalid config");
    /// assert!(!error.is_recoverable()); // Configuration errors typically aren't recoverable
    /// ```
    pub fn is_recoverable(&self) -> bool {
        match self {
            ZynapseError::Io { .. } => true,
            ZynapseError::Configuration { .. } => false,
            ZynapseError::NoteNotFound { .. } => false,
            ZynapseError::InvalidContent { .. } => false,
            #[cfg(feature = "search")]
            ZynapseError::Search { .. } => true,
            #[cfg(feature = "basic-storage")]
            ZynapseError::Storage { .. } => true,
            ZynapseError::Serialization { .. } => false,
            #[cfg(feature = "cli")]
            ZynapseError::Cli { .. } => false,
            #[cfg(feature = "tui")]
            ZynapseError::Tui { .. } => true,
            ZynapseError::Internal { .. } => false,
        }
    }

    /// Get the error category
    /// エラーカテゴリを取得
    ///
    /// Returns a string describing the general category of the error.
    /// エラーの一般的なカテゴリを説明する文字列を返します。
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zynapse::ZynapseError;
    ///
    /// let error = ZynapseError::config_error("Invalid config");
    /// assert_eq!(error.category(), "Configuration");
    /// ```
    pub fn category(&self) -> &'static str {
        match self {
            ZynapseError::Io { .. } => "I/O",
            ZynapseError::Configuration { .. } => "Configuration",
            ZynapseError::NoteNotFound { .. } => "NotFound",
            ZynapseError::InvalidContent { .. } => "InvalidContent",
            #[cfg(feature = "search")]
            ZynapseError::Search { .. } => "Search",
            #[cfg(feature = "basic-storage")]
            ZynapseError::Storage { .. } => "Storage",
            ZynapseError::Serialization { .. } => "Serialization",
            #[cfg(feature = "cli")]
            ZynapseError::Cli { .. } => "CLI",
            #[cfg(feature = "tui")]
            ZynapseError::Tui { .. } => "TUI",
            ZynapseError::Internal { .. } => "Internal",
        }
    }
}

// Conversion implementations for common error types
// 一般的なエラー型への変換実装

impl From<std::io::Error> for ZynapseError {
    fn from(error: std::io::Error) -> Self {
        Self::io_error(error, "I/O operation failed")
    }
}

impl From<serde_yaml::Error> for ZynapseError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Serialization {
            message: "YAML serialization failed".to_string(),
            source: Box::new(error),
        }
    }
}

impl From<serde_json::Error> for ZynapseError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization {
            message: "JSON serialization failed".to_string(),
            source: Box::new(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_creation() {
        let error = ZynapseError::config_error("Test error");
        assert_eq!(error.category(), "Configuration");
        assert!(!error.is_recoverable());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let zynapse_error: ZynapseError = io_error.into();

        assert_eq!(zynapse_error.category(), "I/O");
        assert!(zynapse_error.is_recoverable());
    }

    #[test]
    fn test_note_not_found() {
        let error = ZynapseError::note_not_found("test-note-123");
        assert_eq!(error.category(), "NotFound");

        match error {
            ZynapseError::NoteNotFound { id } => {
                assert_eq!(id, "test-note-123");
            }
            _ => panic!("Expected NoteNotFound error"),
        }
    }

    #[test]
    fn test_error_display() {
        let error = ZynapseError::invalid_content("Empty content not allowed");
        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid note content"));
        assert!(error_string.contains("Empty content not allowed"));
    }

    #[test]
    fn test_recoverable_errors() {
        let io_error = ZynapseError::io_error(
            io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            "Failed to write file",
        );
        assert!(io_error.is_recoverable());

        let config_error = ZynapseError::config_error("Invalid syntax");
        assert!(!config_error.is_recoverable());
    }
}
