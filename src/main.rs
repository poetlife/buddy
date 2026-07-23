use clap::Parser;

/// 个人向 CLI 辅助工具
#[derive(Parser)]
#[command(name = "buddy", version, about)]
struct Cli;

fn main() {
    let _cli = Cli::parse();
}
