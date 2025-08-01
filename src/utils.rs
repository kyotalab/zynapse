//! Utility functions and helpers for Zynapse
//! Zynapseのユーティリティ関数とヘルパー
//!
//! This module contains common utility functions used throughout the Zynapse codebase,
//! including string manipulation, file operations, and validation helpers.
//! このモジュールはZynapseコードベース全体で使用される一般的なユーティリティ関数を含み、
//! 文字列操作、ファイル操作、検証ヘルパーが含まれます。

use crate::{Result, ZynapseError};
use std::path::Path;

/// Sanitize a string for use as a filename
/// ファイル名として使用するための文字列をサニタイズ
///
/// Removes or replaces characters that are invalid in filenames across different
/// operating systems, while preserving readability.
/// 異なるオペレーティングシステムでファイル名として無効な文字を削除または置換し、
/// 可読性を保持します。
///
/// # Arguments
/// # 引数
///
/// * `input` - The string to sanitize / サニタイズする文字列
///
/// # Returns
/// # 戻り値
///
/// A sanitized string safe for use as a filename
/// ファイル名として安全に使用できるサニタイズされた文字列
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::sanitize_filename;
///
/// assert_eq!(sanitize_filename("Hello World!"), "hello-world");
/// assert_eq!(sanitize_filename("Test/File:Name"), "test-file-name");
/// assert_eq!(sanitize_filename("日本語テスト"), "日本語テスト");
/// ```
#[must_use]
pub fn sanitize_filename(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| match c {
            // Keep alphanumeric, underscores, hyphens, and Unicode letters
            // 英数字、アンダースコア、ハイフン、Unicode文字を保持
            c if c.is_alphanumeric() || c == '_' || c == '-' => c,
            // Replace whitespace and common punctuation with hyphens
            // 空白と一般的な句読点をハイフンに置換
            ' ' | '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '.' => '-',
            // Remove other problematic characters
            // その他の問題のある文字を削除
            _ => '\0',
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

/// Generate a unique identifier based on content
/// 内容に基づいて一意識別子を生成
///
/// Creates a short hash of the input content that can be used as a unique identifier.
/// 一意識別子として使用できる入力内容の短いハッシュを作成します。
///
/// # Arguments
/// # 引数
///
/// * `content` - The content to hash / ハッシュする内容
///
/// # Returns
/// # 戻り値
///
/// An 8-character hexadecimal hash string
/// 8文字の16進ハッシュ文字列
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::generate_content_hash;
///
/// let hash1 = generate_content_hash("Hello, World!");
/// let hash2 = generate_content_hash("Hello, World!");
/// let hash3 = generate_content_hash("Different content");
///
/// assert_eq!(hash1, hash2); // Same content produces same hash
/// assert_ne!(hash1, hash3); // Different content produces different hash
/// assert_eq!(hash1.len(), 8); // Hash is always 8 characters
/// ```
#[must_use]
pub fn generate_content_hash(content: &str) -> String {
    let hash = blake3::hash(content.as_bytes());
    hex::encode(&hash.as_bytes()[..4])
}

/// Extract a title from markdown content
/// Markdownコンテンツからタイトルを抽出
///
/// Attempts to extract a title from markdown content by looking for:
/// 以下を探してMarkdownコンテンツからタイトルの抽出を試みます：
/// 1. First H1 heading (# Title)
/// 2. First H2 heading (## Title) if no H1 found
/// 3. First line if no headings found
///
/// # Arguments
/// # 引数
///
/// * `content` - The markdown content / Markdownコンテンツ
///
/// # Returns
/// # 戻り値
///
/// Extracted title or a default if none found
/// 抽出されたタイトル、見つからない場合はデフォルト
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::extract_title_from_content;
///
/// let content1 = "# Main Title\n\nSome content here";
/// assert_eq!(extract_title_from_content(content1), "main-title");
///
/// let content2 = "## Subtitle\n\nContent without H1";
/// assert_eq!(extract_title_from_content(content2), "subtitle");
///
/// let content3 = "Just plain text content";
/// assert_eq!(extract_title_from_content(content3), "just-plain-text-content");
/// ```
#[must_use]
pub fn extract_title_from_content(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();

    // Look for H1 heading first
    // まずH1見出しを探す
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let title = trimmed.trim_start_matches("# ").trim();
            if !title.is_empty() {
                return sanitize_filename(title);
            }
        }
    }

    // Look for H2 heading if no H1 found
    // H1が見つからない場合はH2見出しを探す
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            let title = trimmed.trim_start_matches("## ").trim();
            if !title.is_empty() {
                return sanitize_filename(title);
            }
        }
    }

    // Use first non-empty line as fallback
    // フォールバックとして最初の空でない行を使用
    for line in &lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            // Take first 50 characters to avoid overly long filenames
            // ファイル名が長くなりすぎないよう最初の50文字を取得
            let title = if trimmed.len() > 50 {
                &trimmed[..50]
            } else {
                trimmed
            };
            return sanitize_filename(title);
        }
    }

    // Default fallback
    // デフォルトフォールバック
    "untitled".to_string()
}

/// Validate that a path is safe for file operations
/// パスがファイル操作に安全であることを検証
///
/// Checks that a path doesn't contain dangerous patterns that could
/// lead to directory traversal attacks or access to system files.
/// パスがディレクトリトラバーサル攻撃やシステムファイルへのアクセスに
/// つながる可能性のある危険なパターンを含まないことをチェックします。
///
/// # Arguments
/// # 引数
///
/// * `path` - The path to validate / 検証するパス
///
/// # Errors
///
/// Returns an error if the path contains dangerous patterns.
/// パスが危険なパターンを含む場合にエラーを返します。
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::validate_safe_path;
/// use std::path::Path;
///
/// assert!(validate_safe_path(Path::new("notes/my-note.md")).is_ok());
/// assert!(validate_safe_path(Path::new("../../../etc/passwd")).is_err());
/// assert!(validate_safe_path(Path::new("C:\\Windows\\System32")).is_err());
/// ```
pub fn validate_safe_path(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();

    // Check for directory traversal patterns
    // ディレクトリトラバーサルパターンをチェック
    if path_str.contains("..") {
        return Err(ZynapseError::invalid_content(format!(
            "Path contains directory traversal: {path_str}"
        )));
    }

    // Check for absolute paths to system directories (Unix)
    // システムディレクトリへの絶対パス（Unix）をチェック
    if path_str.starts_with("/etc/")
        || path_str.starts_with("/sys/")
        || path_str.starts_with("/proc/")
        || path_str.starts_with("/dev/")
    {
        return Err(ZynapseError::invalid_content(format!(
            "Path accesses system directory: {path_str}"
        )));
    }

    // Check for Windows system paths
    // Windowsシステムパスをチェック
    if path_str.to_lowercase().starts_with("c:\\windows")
        || path_str.to_lowercase().starts_with("c:\\system")
    {
        return Err(ZynapseError::invalid_content(format!(
            "Path accesses Windows system directory: {path_str}"
        )));
    }

    Ok(())
}

/// Format file size in human-readable format
/// ファイルサイズを人間が読みやすい形式でフォーマット
///
/// Converts byte sizes into human-readable format with appropriate units.
/// バイトサイズを適切な単位で人間が読みやすい形式に変換します。
///
/// # Arguments
/// # 引数
///
/// * `bytes` - Size in bytes / バイト単位のサイズ
///
/// # Returns
/// # 戻り値
///
/// Formatted size string (e.g., "1.5 KB", "2.3 MB")
/// フォーマットされたサイズ文字列（例："1.5 KB", "2.3 MB"）
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::format_file_size;
///
/// assert_eq!(format_file_size(0), "0 B");
/// assert_eq!(format_file_size(1024), "1.0 KB");
/// assert_eq!(format_file_size(1536), "1.5 KB");
/// assert_eq!(format_file_size(1048576), "1.0 MB");
/// ```
#[must_use]
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    // Use explicit casting with allow to acknowledge precision loss
    // 精度損失を承知で明示的キャストを使用
    #[allow(clippy::cast_precision_loss)]
    let bytes_f = bytes as f64;
    let mut size = bytes_f;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Get current timestamp in ISO 8601 format
/// ISO 8601形式で現在のタイムスタンプを取得
///
/// Returns the current UTC time formatted as an ISO 8601 string.
/// 現在のUTC時間をISO 8601文字列として返します。
///
/// # Returns
/// # 戻り値
///
/// ISO 8601 formatted timestamp string
/// ISO 8601形式のタイムスタンプ文字列
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::current_timestamp;
///
/// let timestamp = current_timestamp();
/// assert!(timestamp.contains("T"));
/// assert!(timestamp.ends_with('Z') || timestamp.contains('+') || timestamp.contains('-'));
/// ```
#[must_use]
pub fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Check if a string is empty or contains only whitespace
/// 文字列が空または空白のみを含むかチェック
///
/// # Arguments
/// # 引数
///
/// * `input` - The string to check / チェックする文字列
///
/// # Returns
/// # 戻り値
///
/// True if the string is empty or whitespace-only
/// 文字列が空または空白のみの場合にtrue
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::is_empty_or_whitespace;
///
/// assert!(is_empty_or_whitespace(""));
/// assert!(is_empty_or_whitespace("   "));
/// assert!(is_empty_or_whitespace("\t\n  "));
/// assert!(!is_empty_or_whitespace("Hello"));
/// assert!(!is_empty_or_whitespace("  Hello  "));
/// ```
#[must_use]
pub fn is_empty_or_whitespace(input: &str) -> bool {
    input.trim().is_empty()
}

/// Truncate a string to a specified length with ellipsis
/// 文字列を指定の長さに省略記号付きで切り詰め
///
/// Truncates a string to the specified maximum length, adding "..." if truncated.
/// 文字列を指定の最大長に切り詰め、切り詰められた場合は"..."を追加します。
///
/// # Arguments
/// # 引数
///
/// * `input` - The string to truncate / 切り詰める文字列
/// * `max_length` - Maximum length including ellipsis / 省略記号を含む最大長
///
/// # Returns
/// # 戻り値
///
/// Truncated string with ellipsis if needed
/// 必要に応じて省略記号付きの切り詰められた文字列
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::truncate_string;
///
/// assert_eq!(truncate_string("Hello, World!", 10), "Hello, ...");
/// assert_eq!(truncate_string("Short", 10), "Short");
/// assert_eq!(truncate_string("Exact", 5), "Exact");
/// ```
#[must_use]
pub fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else if max_length <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &input[..max_length - 3])
    }
}

/// Normalize line endings to Unix style (LF)
/// 行末をUnixスタイル（LF）に正規化
///
/// Converts Windows (CRLF) and classic Mac (CR) line endings to Unix (LF).
/// Windows（CRLF）と古いMac（CR）の行末をUnix（LF）に変換します。
///
/// # Arguments
/// # 引数
///
/// * `input` - The string to normalize / 正規化する文字列
///
/// # Returns
/// # 戻り値
///
/// String with normalized line endings
/// 正規化された行末の文字列
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::normalize_line_endings;
///
/// assert_eq!(normalize_line_endings("Line1\r\nLine2"), "Line1\nLine2");
/// assert_eq!(normalize_line_endings("Line1\rLine2"), "Line1\nLine2");
/// assert_eq!(normalize_line_endings("Line1\nLine2"), "Line1\nLine2");
/// ```
#[must_use]
pub fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

/// Create a backup filename with timestamp
/// タイムスタンプ付きのバックアップファイル名を作成
///
/// Creates a backup filename by inserting a timestamp before the file extension.
/// ファイル拡張子の前にタイムスタンプを挿入してバックアップファイル名を作成します。
///
/// # Arguments
/// # 引数
///
/// * `original_path` - The original file path / 元のファイルパス
///
/// # Returns
/// # 戻り値
///
/// Path with timestamp inserted before extension
/// 拡張子の前にタイムスタンプが挿入されたパス
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::create_backup_filename;
/// use std::path::Path;
///
/// let original = Path::new("note.md");
/// let backup = create_backup_filename(original);
///
/// // Result will be something like "note_20231201_143025.md"
/// // 結果は"note_20231201_143025.md"のようになります
/// assert!(backup.to_string_lossy().contains("note_"));
/// assert!(backup.to_string_lossy().ends_with(".md"));
/// ```
#[must_use]
pub fn create_backup_filename(original_path: &Path) -> std::path::PathBuf {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

    match (original_path.file_stem(), original_path.extension()) {
        (Some(stem), Some(ext)) => {
            let mut backup_name = stem.to_os_string();
            backup_name.push(format!("_{timestamp}"));
            original_path
                .with_file_name(backup_name)
                .with_extension(ext)
        }
        (Some(stem), None) => {
            let mut backup_name = stem.to_os_string();
            backup_name.push(format!("_{timestamp}"));
            original_path.with_file_name(backup_name)
        }
        (None, _) => original_path.with_file_name(format!("backup_{timestamp}")),
    }
}

/// Ensure a directory exists, creating it if necessary
/// ディレクトリが存在することを確認し、必要に応じて作成
///
/// Creates the directory and all necessary parent directories if they don't exist.
/// ディレクトリが存在しない場合、ディレクトリと必要なすべての親ディレクトリを作成します。
///
/// # Arguments
/// # 引数
///
/// * `path` - The directory path to ensure / 確認するディレクトリパス
///
/// # Errors
///
/// Returns an error if the directory cannot be created.
/// ディレクトリが作成できない場合にエラーを返します。
///
/// # Examples
///
/// ```rust,no_run
/// use zynapse::utils::ensure_directory_exists;
/// use std::path::Path;
///
/// ensure_directory_exists(Path::new("/tmp/zynapse/notes"))?;
/// # Ok::<(), zynapse::ZynapseError>(())
/// ```
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|e| {
            ZynapseError::io_error(e, format!("Failed to create directory: {path:?}"))
        })?;
    } else if !path.is_dir() {
        return Err(ZynapseError::invalid_content(format!(
            "Path exists but is not a directory: {path:?}"
        )));
    }
    Ok(())
}

/// Calculate the relative path from one directory to another
/// あるディレクトリから別のディレクトリへの相対パスを計算
///
/// # Arguments
/// # 引数
///
/// * `from` - The source directory / ソースディレクトリ
/// * `to` - The target path / ターゲットパス
///
/// # Returns
/// # 戻り値
///
/// Relative path from source to target
/// ソースからターゲットへの相対パス
///
/// # Examples
///
/// ```rust
/// use zynapse::utils::relative_path;
/// use std::path::Path;
///
/// let from = Path::new("/home/user/notes");
/// let to = Path::new("/home/user/notes/2023/note.md");
/// let relative = relative_path(from, to);
/// assert_eq!(relative.to_str().unwrap(), "2023/note.md");
/// ```
#[must_use]
pub fn relative_path(from: &Path, to: &Path) -> std::path::PathBuf {
    to.strip_prefix(from)
        .map_or_else(|_| to.to_path_buf(), Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Hello World!"), "hello-world");
        assert_eq!(sanitize_filename("Test/File:Name"), "test-file-name");
        assert_eq!(sanitize_filename("Multiple   Spaces"), "multiple-spaces");
        assert_eq!(sanitize_filename(""), "");
        assert_eq!(sanitize_filename("---test---"), "test");

        // Test consecutive characters that become hyphens
        // ハイフンになる連続文字のテスト
        assert_eq!(sanitize_filename("test...file"), "test-file");
        assert_eq!(sanitize_filename("a////b"), "a-b");

        // Test Unicode characters are preserved
        // Unicode文字が保持されることをテスト
        assert_eq!(sanitize_filename("日本語テスト"), "日本語テスト");
        assert_eq!(sanitize_filename("Hello 世界"), "hello-世界");
    }

    #[test]
    fn test_generate_content_hash() {
        let content1 = "Hello, World!";
        let content2 = "Hello, World!";
        let content3 = "Different content";

        let hash1 = generate_content_hash(content1);
        let hash2 = generate_content_hash(content2);
        let hash3 = generate_content_hash(content3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 8);
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_extract_title_from_content() {
        // Test H1 heading
        let content1 = "# Main Title\n\nSome content here";
        assert_eq!(extract_title_from_content(content1), "main-title");

        // Test H2 heading when no H1
        let content2 = "## Subtitle\n\nContent without H1";
        assert_eq!(extract_title_from_content(content2), "subtitle");

        // Test first line fallback
        let content3 = "Just plain text content";
        assert_eq!(
            extract_title_from_content(content3),
            "just-plain-text-content"
        );

        // Test empty content
        let content4 = "";
        assert_eq!(extract_title_from_content(content4), "untitled");

        // Test long content truncation
        let long_content = "This is a very long line that should be truncated to avoid overly long filenames that could cause issues";
        let result = extract_title_from_content(long_content);
        assert!(result.len() <= 50);
    }

    #[test]
    fn test_validate_safe_path() {
        // Safe paths
        assert!(validate_safe_path(Path::new("notes/my-note.md")).is_ok());
        assert!(validate_safe_path(Path::new("folder/subfolder/file.txt")).is_ok());

        // Dangerous paths
        assert!(validate_safe_path(Path::new("../../../etc/passwd")).is_err());
        assert!(validate_safe_path(Path::new("/etc/shadow")).is_err());
        assert!(validate_safe_path(Path::new("C:\\Windows\\System32")).is_err());
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_is_empty_or_whitespace() {
        assert!(is_empty_or_whitespace(""));
        assert!(is_empty_or_whitespace("   "));
        assert!(is_empty_or_whitespace("\t\n  "));
        assert!(!is_empty_or_whitespace("Hello"));
        assert!(!is_empty_or_whitespace("  Hello  "));
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello, World!", 10), "Hello, ...");
        assert_eq!(truncate_string("Short", 10), "Short");
        assert_eq!(truncate_string("Exact", 5), "Exact");
        assert_eq!(truncate_string("Too long", 3), "...");
    }

    #[test]
    fn test_normalize_line_endings() {
        assert_eq!(normalize_line_endings("Line1\r\nLine2"), "Line1\nLine2");
        assert_eq!(normalize_line_endings("Line1\rLine2"), "Line1\nLine2");
        assert_eq!(normalize_line_endings("Line1\nLine2"), "Line1\nLine2");
    }

    #[test]
    fn test_create_backup_filename() {
        let original = Path::new("test.md");
        let backup = create_backup_filename(original);

        let backup_str = backup.to_string_lossy();
        assert!(backup_str.starts_with("test_"));
        assert!(backup_str.ends_with(".md"));
        assert!(backup_str.len() > "test.md".len());
    }

    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("new_directory");

        assert!(!test_path.exists());
        assert!(ensure_directory_exists(&test_path).is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());

        // Test idempotency
        assert!(ensure_directory_exists(&test_path).is_ok());
    }

    #[test]
    fn test_relative_path() {
        let from = Path::new("/home/user/notes");
        let to = Path::new("/home/user/notes/2023/note.md");
        let relative = relative_path(from, to);
        assert_eq!(relative, Path::new("2023/note.md"));

        // Test when paths don't share common prefix
        let from2 = Path::new("/different/path");
        let relative2 = relative_path(from2, to);
        assert_eq!(relative2, to);
    }

    #[test]
    fn test_current_timestamp() {
        let timestamp = current_timestamp();
        assert!(timestamp.contains('T'));
        assert!(timestamp.ends_with('Z') || timestamp.contains('+') || timestamp.contains('-'));

        // Should be valid RFC3339 format
        assert!(chrono::DateTime::parse_from_rfc3339(&timestamp).is_ok());
    }
}
