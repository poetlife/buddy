use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

use chrono::Local;
use serde::Serialize;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use uuid::Uuid;

fn audit_base_dir() -> PathBuf {
    if let Ok(dir) = env::var("BUDDY_AUDIT_DIR") {
        PathBuf::from(dir)
    } else {
        let home = env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".buddy").join("audit")
    }
}

/// 持有日志初始化后的状态，负责写摘要和打印终端 epilogue。
pub struct LoggerGuard {
    trace_id: String,
    audit_dir: PathBuf,
    start: Instant,
    finalized: bool,
}

/// 初始化日志系统。
///
/// `verbose` 为 `true` 时向 stderr 注册人类可读的控制台 subscriber，
/// 否则仅注册文件的 JSON Lines subscriber。
pub fn init(verbose: bool) -> LoggerGuard {
    let trace_id = Uuid::new_v4().simple().to_string();
    let now = Local::now();
    let dir_name = format!("{}-{}", now.format("%Y%m%d-%H%M%S"), &trace_id[..8]);
    let audit_dir = audit_base_dir().join(&dir_name);
    fs::create_dir_all(&audit_dir).expect("failed to create audit directory");

    let file_appender = tracing_appender::rolling::never(&audit_dir, "log.jsonl");
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file_appender);

    if verbose {
        let console_layer = tracing_subscriber::fmt::layer().with_writer(io::stderr);
        Registry::default()
            .with(file_layer)
            .with(console_layer)
            .try_init()
            .ok();
    } else {
        Registry::default()
            .with(file_layer)
            .try_init()
            .ok();
    }

    LoggerGuard {
        trace_id,
        audit_dir,
        start: Instant::now(),
        finalized: false,
    }
}

impl LoggerGuard {
    /// 当前调用的 trace ID。
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    /// 写入 `summary.json` 并打印终端摘要。
    ///
    /// `exit_code` 为 0 表示成功，非 0 表示失败。
    /// 调用后 guard 被消费，不会再次写摘要。
    pub fn finalize(mut self, exit_code: i32) {
        self.do_finalize(exit_code);
        self.finalized = true;
    }

    fn do_finalize(&self, exit_code: i32) {
        let elapsed = self.start.elapsed();
        let success = exit_code == 0;

        #[derive(Serialize)]
        struct Summary<'a> {
            trace_id: &'a str,
            status: &'a str,
            exit_code: i32,
            elapsed_ms: u128,
            audit_dir: &'a str,
        }

        let summary = Summary {
            trace_id: &self.trace_id,
            status: if success { "SUCCESS" } else { "FAILED" },
            exit_code,
            elapsed_ms: elapsed.as_millis(),
            audit_dir: self.audit_dir.to_str().unwrap_or(""),
        };

        if let Ok(json) = serde_json::to_string_pretty(&summary) {
            let _ = fs::write(self.audit_dir.join("summary.json"), json);
        }

        let status = if success { "SUCCESS" } else { "FAILED" };
        eprintln!(
            "status={} trace_id={} elapsed={:.2}s audit={}",
            status,
            self.trace_id,
            elapsed.as_secs_f64(),
            self.audit_dir.display(),
        );
    }
}

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        if !self.finalized {
            self.do_finalize(1);
        }
    }
}
