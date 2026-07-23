# buddy

## 简介

个人向 CLI 辅助工具，基于 Rust + clap 实现，提供日常工作和开发中的辅助功能。

## 快速开始

### 环境要求

- Rust 工具链（stable）

### 安装

```bash
cargo build --release
```

### 运行

```bash
cargo run -- <subcommand>
```

## 项目结构

```
buddy/
├── src/                       # 源代码
│   └── main.rs                # 程序入口
├── docs/                      # 项目文档
│   ├── design/                # 功能/模块设计文档（spec）
│   ├── debugging/             # 排障记录
│   │   ├── registry.md        # 排障索引
│   │   └── records/           # 历史排障记录
│   ├── observability.md       # 可观测性规范
│   ├── testing.md             # 测试指南
│   └── ssot-registry.md       # SSOT 注册表
├── CODEBUDDY.md               # 项目入口与 AI 协作规范
└── README.md                  # 本文件
```

## 开发规范

见 [CODEBUDDY.md](CODEBUDDY.md)。
