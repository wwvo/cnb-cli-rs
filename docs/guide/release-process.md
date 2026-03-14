# 发布流程

`cnb` 项目当前采用“两段式”发版流程：

- `release PR` 负责更新项目版本与 `CHANGELOG.md`
- `tag release` 负责校验、创建 Release、构建并上传多平台附件

这样可以兼容受保护的 `main` 分支，避免正式发布流程直接向主干推送提交。

## 发布准备

每次正式发版前，先从 `main` 拉出一个 release 分支，例如：

```bash
git checkout main
git pull --ff-only
git checkout -b chore/release-v0.1.0
```

然后在该分支完成以下准备工作：

1. 更新根目录 `Cargo.toml` 中 `[workspace.package].version`
2. 生成完整项目级 `CHANGELOG.md`
3. 提交并创建指向 `main` 的 PR

生成 `CHANGELOG.md` 的推荐命令：

```bash
git cliff -o CHANGELOG.md
```

`CHANGELOG.md` 是项目历史文档，应在 PR 中接受审阅后再合入 `main`。

## 正式发布

release PR 合并到 `main` 后，从合并后的提交打版本标签：

```bash
git checkout main
git pull --ff-only
git tag v0.1.0
git push origin v0.1.0
```

标签推送后，CNB 流水线会自动执行以下步骤：

1. 运行发版前检查
2. 校验标签格式与 `Cargo.toml` 版本一致
3. 运行 `cargo test --workspace --target x86_64-unknown-linux-gnu`
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
  - 在 release PR 中生成并提交
  - 进入 `main` 前需要审阅
- `LATEST_CHANGELOG.md`
  - 单次发布说明
  - 由 tag 发布流水线临时生成
  - 作为 Release 描述使用，不回写仓库

## 为什么不在 release 流水线里更新 CHANGELOG.md

仓库的 `main` 分支启用了保护策略。为了避免正式发版时因为 `push main` 失败而阻塞附件上传，release 流水线不会直接提交或推送 `CHANGELOG.md`。

如果需要更新项目级 changelog，请始终通过 release PR 完成。
