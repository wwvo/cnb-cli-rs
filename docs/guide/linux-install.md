# Linux 安装说明

## 适用范围

当前 Linux 原生安装包首轮仅覆盖 `x86_64`，并作为 release 页面附件提供：

- `.deb`
- `.rpm`

如果你的 Linux 环境不适用这两种包格式，仍然可以继续使用 `.tar.gz` 手动解压安装。

> [!NOTE]
> 当前只提供 release 附件下载，不提供 apt / yum 软件源托管。

## 选择哪种安装方式

- Debian / Ubuntu / Linux Mint：优先使用 `.deb`
- Fedora / Rocky / AlmaLinux / RHEL 系：优先使用 `.rpm`
- 其他 Linux 发行版，或你希望自行控制安装目录：使用 `.tar.gz`

## 使用 `.deb` 安装

```bash
sudo dpkg -i ./cnb-rs-<VERSION>-1.x86_64.deb
```

安装后默认会包含：

- `/usr/bin/cnb-rs`
- Bash 补全：`/usr/share/bash-completion/completions/cnb-rs`
- Zsh 补全：`/usr/share/zsh/vendor-completions/_cnb-rs`
- Fish 补全：`/usr/share/fish/vendor_completions.d/cnb-rs.fish`

## 使用 `.rpm` 安装

```bash
sudo dnf install ./cnb-rs-<VERSION>-1.x86_64.rpm
```

如果你使用的 RPM 系环境更偏底层，也可以改用：

```bash
sudo rpm -i ./cnb-rs-<VERSION>-1.x86_64.rpm
```

安装内容与 `.deb` 保持一致，包括 `cnb-rs` 可执行文件和 Bash / Zsh / Fish 补全文件。

## 使用 `.tar.gz` 手动安装

```bash
tar -xzf ./cnb-rs-<VERSION>-x86_64-unknown-linux-gnu.tar.gz
sudo install -m 755 ./cnb-rs-<VERSION>-x86_64-unknown-linux-gnu/cnb-rs /usr/local/bin/cnb-rs
```

这种方式不会自动安装 Bash / Zsh / Fish 补全文件；如果需要，请参考 [cnb-rs completion](/completion) 手动生成。

## 验证安装

```bash
cnb-rs --version
cnb-rs auth status
```

如果命令找不到，请先确认：

- `cnb-rs` 已位于 `PATH` 中
- 当前 shell 已重新加载
- 你使用的是 `cnb-rs`，而不是旧命令名 `cnb`

旧命令迁移说明见：[从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)。
