# 快速开始

## 安装

从 [Release 页面](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/releases) 下载适合你平台的发布产物。

### Linux

首轮 Linux 原生包当前仅覆盖 `x86_64`：

```bash
# Debian / Ubuntu
sudo dpkg -i ./cnb-rs-<VERSION>-1.x86_64.deb

# Fedora / Rocky / AlmaLinux
sudo dnf install ./cnb-rs-<VERSION>-1.x86_64.rpm
```

如果你的环境不适合 `.deb` / `.rpm`，也可以继续下载 `.tar.gz` 手动解压，并把 `cnb-rs` 放到 `PATH` 中。

> [!NOTE]
> 当前只提供 release 附件下载，不提供 apt / yum 软件源托管。

更多说明见：[Linux 安装说明](/guide/linux-install)。

### Windows / macOS

Windows 与 macOS 当前继续使用 release 页面中的压缩包附件。

> [!WARNING]
> 从改名版本开始，原来的 `cnb ...` 已改为 `cnb-rs ...`。
> 如果你是从旧版本升级，请先阅读 [从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)。

## 登录

```bash
cnb-rs auth login
# 或直接指定 Token
cnb-rs auth login --token <YOUR_TOKEN>
```

## 基本使用

```bash
# 查看当前认证状态
cnb-rs auth status

# 查看仓库信息
cnb-rs info

# 查看 Issue 列表
cnb-rs issue list

# 使用 AI 对话
cnb-rs chat --do "查看我的 Issue 列表"
```

## 升级自旧版本

- 旧命令：`cnb ...`
- 新命令：`cnb-rs ...`
- 如果你想保留旧输入习惯，请自行配置 shell alias
- Linux 用户若改用 `.deb` / `.rpm` 安装，补全文件会随包一起安装，无需再手工拷贝 Bash / Zsh / Fish 补全脚本

完整迁移说明见：[从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)
