//! Basic usage example for Zynapse CLI
//! `ZynapseCLIの基本使用例`
//!
//! This example demonstrates the basic CLI functionality of Zynapse.
//! `この例はZynapseの基本CLI機能を実演します`。

#[cfg(feature = "cli")]
fn main() {
    println!("🚀 Zynapse Basic Usage Example");
    println!("================================");
    println!();
    println!("This example will demonstrate:");
    println!("この例では以下を実演します：");
    println!("• Note creation (ノート作成)");
    println!("• Basic search (基本検索)");
    println!("• TUI interface (TUIインターフェース)");
    println!();
    println!("⚠️  Implementation coming in Phase 1 development");
    println!("   Phase 1開発で実装予定");
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("❌ This example requires the 'cli' feature.");
    println!("   この例には'cli'機能が必要です。");
    println!("   Run with: cargo run --example basic_usage --features cli");
}
