# 发布流程

`cnb` 项目当前采用“准备发版 PR -> 合并后自动打 tag -> tag 自动发布”的三段式流程：

- `release prepare` 负责自动推导下一个版本号、更新 `Cargo.toml` 与 `CHANGELOG.md`、创建 release PR
- `main push` 负责在 release PR 合并后自动创建对应 tag
- `tag release` 负责校验、创建 Release、构建并上传多平台附件

这样既兼容受保护的 `main` 分支，也把手工 bump version、手工建 release PR、手工打 tag 这几步收敛进了流水线。

## 1. 准备 release PR

在 `main` 分支页面点击 `准备 release PR` 按钮后，流水线会自动执行：

1. 读取当前 `Cargo.toml` 中的 workspace version
2. 查找对应的 `v<current_version>` 标签
3. 基于该标签以来的提交自动推导版本升级级别
4. 自动 bump 到下一个版本
5. 重新生成项目级 `CHANGELOG.md`
6. 推送 `release/vX.Y.Z` 分支
7. 自动创建指向 `main` 的 release PR

版本升级规则：

- 检测到 `BREAKING CHANGE` 或 `type!:` 时视为 `major`
- 检测到 `feat` 时视为 `minor`
- 其他提交默认视为 `patch`
- 对 `0.x` 版本，即使检测到 `major`，也会按 `minor` 处理，避免直接跳到 `1.0.0`

release PR 仍然需要正常 review；`CHANGELOG.md` 作为项目历史文档，会在这个 PR 中被审阅后进入 `main`。

## 2. main 合并后自动打 tag

release PR 合并到 `main` 后，`main push` 流水线会自动执行：

1. 运行 `rustfmt`、`cargo check`、`cargo clippy`、`cargo test`
2. 在代码检查通过后，检查本次提交是否同时修改了 `Cargo.toml` 与 `CHANGELOG.md`
3. 如果当前 workspace version 对应的 tag 不存在，则自动创建 `vX.Y.Z`

这一步不再需要手工执行 `git tag` 和 `git push origin <tag>`。

## 3. tag 自动正式发布

标签创建后，CNB 流水线会自动执行以下步骤：

1. 运行发版前检查
2. 校验标签格式与 `Cargo.toml` 版本一致
3. 运行 release 镜像内的 `cargo test --workspace --target x86_64-unknown-linux-gnu`
4. 生成本次发布说明 `LATEST_CHANGELOG.md`
5. 创建 Release
6. 构建并上传 Linux、Windows、macOS 附件

## 标签规则

发布标签必须满足以下要求：

- 以 `v` 开头
- 符合 `vX.Y.Z`、`vX.Y.Z-alpha.1`、`vX.Y.Z-beta.1` 这类格式
- 去掉前缀 `v` 后，版本号必须与根目录 `Cargo.toml` 中的 workspace version 完全一致

不符合规则的标签会在发版前检查阶段直接失败。

## CHANGELOG 约定

项目中有两类 changelog 产物：

- `CHANGELOG.md`
  - 项目级历史文档
  - 在 `release-prepare` 流水线中生成并提交到 release PR
  - 进入 `main` 前需要审阅
- `LATEST_CHANGELOG.md`
  - 单次发布说明
  - 由 tag 发布流水线临时生成
  - 作为 Release 描述使用，不回写仓库

## 为什么 release 流程要拆成三段

仓库的 `main` 分支启用了保护策略。如果在正式 release 流水线中直接修改或推送 `main`，一旦分支保护拦截，就会连带阻塞附件上传。

拆成“release PR / main 自动打 tag / tag 正式发布”之后：

- 项目级变更文档仍然通过 PR 审阅进入 `main`
- 正式 release 流水线不再直接写主干
- release PR 合并后无需再手工打 tag
