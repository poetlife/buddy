use clap::Parser;
use tracing::info_span;

mod logging;

/// 个人向 CLI 辅助工具
#[derive(Parser)]
#[command(name = "buddy", version, about)]
struct Cli {
    /// 输出详细日志到终端
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    let guard = logging::init(cli.verbose);

    let exit_code = info_span!("buddy", trace_id = guard.trace_id()).in_scope(|| {
        tracing::info!("buddy started");
        0
    });

    guard.finalize(exit_code);
}
