# GitHub Actions Workflow 设计说明

本文档说明 `cnb cli` 项目的 GitHub Actions Workflow 设计与实现细节。

## 概述

本项目使用 GitHub Actions 实现多操作系统与 CPU 架构的交叉编译与产物发布，覆盖 **10 种平台/架构组合**。

## 矩阵配置

### 支持的目标平台

| 平台 | Target | 架构 | 运行环境 | 编译方式 |
|------|--------|------|----------|----------|
| Windows MSVC | x86_64-pc-windows-msvc | x86_64 | windows-latest | cargo |
| Windows MSVC | aarch64-pc-windows-msvc | aarch64 | windows-latest | cargo |
| Windows GNU | x86_64-pc-windows-gnu | x86_64 | windows-latest | cargo + MinGW |
| Windows GNU | aarch64-pc-windows-gnu | aarch64 | ubuntu-latest | cross |
| macOS Intel | x86_64-apple-darwin | x86_64 | macos-latest | cargo |
| macOS Apple Silicon | aarch64-apple-darwin | aarch64 | macos-latest | cargo |
| Linux glibc | x86_64-unknown-linux-gnu | x86_64 | ubuntu-latest | cargo |
| Linux glibc | aarch64-unknown-linux-gnu | aarch64 | ubuntu-latest | cross |
| Linux musl | x86_64-unknown-linux-musl | x86_64 | ubuntu-latest | cross |
| Linux musl | aarch64-unknown-linux-musl | aarch64 | ubuntu-latest | cross |

### 矩阵策略说明

Workflow 使用 `strategy.matrix.include` 定义具体的构建配置，每个配置项包含：

- `os`: GitHub Actions 运行环境
- `target`: Rust 编译目标
- `arch`: CPU 架构
- `platform`: 平台标识（用于产物命名）
- `use_cross`: 是否使用 cross 工具（可选）

## 依赖处理

### Windows MSVC

- 使用 `dtolnay/rust-toolchain` 自动安装 MSVC target
- 无需额外配置，Windows 运行环境已包含 Visual Studio Build Tools

### Windows GNU

**x86_64**:
- 使用 `msys2/setup-msys2` 安装 MinGW-w64 工具链
- 将 MinGW bin 目录添加到 PATH

**aarch64**:
- 使用 `cross` 工具进行交叉编译
- 在 Ubuntu 环境中运行

### macOS

- `macos-latest` 运行环境已包含 Xcode CLI Tools
- 无需额外配置，直接使用 cargo 编译

### Linux glibc

- 标准 Ubuntu 环境直接使用 cargo 编译
- aarch64 使用 cross 进行交叉编译

### Linux musl

- 使用 `cross` 工具进行静态链接编译
- 确保生成的二进制文件无外部依赖

## 构建缓存

使用 `Swatinem/rust-cache` 加速构建：

```yaml
- name: Setup Rust cache
  uses: Swatinem/rust-cache@v2
  with:
    key: ${{ matrix.target }}
```

缓存策略：
- 按 target 分离缓存，避免不同平台缓存冲突
- 缓存 Cargo registry、build artifacts 等

## 产物归档

### 命名规则

```
cnb-{platform}-{arch}.{ext}
```

示例：
- `cnb-windows-msvc-x86_64.zip`
- `cnb-linux-musl-aarch64.tar.gz`

### 包含内容

每个产物包包含两个可执行文件：
- `cnb` / `cnb.exe`: 主程序
- `git-cnb` / `git-cnb.exe`: Git 别名程序

## 触发条件

Workflow 支持以下触发方式：

### Pull Request

```yaml
on:
  pull_request:
    branches: [main]
```

当向 `main` 分支创建或更新 PR 时触发构建。

### Push

```yaml
on:
  push:
    branches: [main]
    tags:
      - 'v*'
```

- 推送到 `main` 分支时触发构建
- 推送 `v*` 标签时触发构建并创建 Release

## 发布流程

当推送 tag 时，`release` job 会自动执行：

1. 下载所有构建产物
2. 使用 `softprops/action-gh-release` 创建 GitHub Release
3. 上传所有产物到 Release

## 扩展建议

### 添加新平台

在 `matrix.include` 中添加新配置项：

```yaml
- os: ubuntu-latest
  target: riscv64gc-unknown-linux-gnu
  arch: riscv64
  platform: linux-gnu
  use_cross: true
```

### 添加测试步骤

在构建完成后添加测试：

```yaml
- name: Run tests
  run: cargo test --target ${{ matrix.target }}
```

### 添加代码签名

在 Windows 和 macOS 构建中添加代码签名步骤以提高安全性。

## 参考链接

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain)
- [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache)
- [cross-rs/cross](https://github.com/cross-rs/cross)
