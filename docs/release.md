# 发版流程

## 版本号约定

本项目遵循 [Semantic Versioning 2.0.0](https://semver.org/)（以下简称 semver），格式为 `MAJOR.MINOR.PATCH`（如 `0.2.1`）。

**版本递增规则**（以当前版本 `0.1.0` 为例）：

| 递增级别 | 示例 | 适用场景 |
|---------|------|---------|
| major | `0.1.0` → `1.0.0` | 不兼容的 API 变更 |
| minor | `0.1.0` → `0.2.0` | 向后兼容的新功能 |
| patch | `0.1.0` → `0.1.1` | 向后兼容的 bug 修复 |

决定递增级别时，看本次发版包含的变更中"最高影响级别"是什么——如果同时有 feat 和 fix，则至少递增 minor。

0.x 版本期间，minor 视为可能 breaking，major 不强制要求 API 变更。

## 发版流程

一次完整的发版包含以下步骤，按顺序执行：

### 1. 核查前置条件

- 确认当前分支为 `main` 且与远程同步
- 确认工作树干净（`git status` 无未提交变更）
- 确认 CI 全部通过
- 确认已合并本版本所有目标 MR/PR

### 2. 查看当前版本

```bash
cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name=="buddy") | .version'
```

或直接查看 `Cargo.toml` 中 `[package]` 下的 `version` 字段。

### 3. 确定新版本号

根据本次包含的变更决定递增级别（major / minor / patch），确定新版本号。

### 4. 更新版本号

修改 `Cargo.toml` 中 `[package].version` 为新版本号。

### 5. 生成 changelog

从上一个 tag 到 HEAD 之间筛选提交，按 Conventional Commits 类型分组，生成 `CHANGELOG.md`。

#### 5.1 获取变更范围

```bash
# 上一个 tag
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
# 若项目尚无 tag，取第一条 commit
[ -z "$LAST_TAG" ] && LAST_TAG=$(git rev-list --max-parents=0 HEAD)
```

#### 5.2 提交类型与 changelog 章节映射

| Conventional Commit 类型 | 对应章节 |
|--------------------------|---------|
| `feat` | Features |
| `fix` | Bug Fixes |
| `perf` | Performance |
| `refactor` | Refactoring |
| `docs` | Documentation |
| `test` | Tests |
| 其他（`chore`、`ci`、`build`、`style`、非标准格式） | Miscellaneous |

#### 5.3 条目格式

每条 changelog 条目格式为：

```
- <提交摘要> (<提交 short-hash>)
```

只取 commit 的第一行（summary），不含 body。同类型内按提交时间倒序排列。

#### 5.4 写入 `CHANGELOG.md`

新版本内容以 prepend 方式插入 `CHANGELOG.md` 头部，格式为：

```markdown
## v<新版本号> (<YYYY-MM-DD>)

### Features

- 功能描述 (abc123)

### Bug Fixes

- 修复描述 (def456)

---
```

保留文件中已有的历史版本内容在下方。

### 6. 提交变更

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to v<新版本号> [skip ci]"
```

commit message 末尾加 `[skip ci]` 以跳过 CI 流程（发版提交不需要再跑一遍）。

### 7. 创建 git tag

```bash
git tag -a v<新版本号> -m "v<新版本号>"
```

- tag 名称固定格式为 `v<版本号>`（如 `v0.2.0`）
- 使用 annotated tag（`-a`），便于追溯创建人和时间
- 若 tag 已存在：停止并排查是否版本号重复

### 8. 推送到远程

```bash
git push
git push --tags
```

推送后确认远程仓库中 tag 可见。

### 9. 发布后验证

- 在远程仓库确认 tag 存在且指向正确 commit
- 确认 `CHANGELOG.md` 内容正确
- 确认 `Cargo.toml` 版本号已更新

## 辅助工具

可使用以下工具辅助执行上述流程（非强制，任选其一）：

- [`cargo-release`](https://github.com/crate-ci/cargo-release): 自动化版本递增、提交、打 tag 和发布
- [`git-cliff`](https://github.com/orhun/git-cliff): 基于 Conventional Commits 自动生成 changelog

后续可能在 `buddy` 自身实现发版辅助命令以统一流程体验。
