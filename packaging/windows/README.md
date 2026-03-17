# Windows MSI Packaging Notes

本目录用于维护 `cnb-rs` 在 Windows 平台上的 MSI 打包配置。

## 安装模型

- 安装范围：`per-machine`
- 默认安装目录：`Program Files\cnb-rs`
- 当前不提供用户级安装模式
- 当前不创建桌面快捷方式或开始菜单快捷方式

## 安装内容

当前 MSI 仅安装以下文件：

- `cnb-rs.exe`
- `LICENSE`
- `README.md`

以下内容不纳入 MSI：

- PowerShell/Bash/Zsh/Fish 补全脚本
- 用户配置文件
- 运行期生成的数据

## PATH 策略

- MSI 会把安装目录追加到系统级 `PATH`
- 卸载时由 Windows Installer 自动移除对应的 PATH 项
- 当前不自动修改 PowerShell `$PROFILE`
- 当前不内置 `cnb` -> `cnb-rs` 的 alias

## 升级与卸载边界

- `UpgradeCode` 固定，后续版本沿用同一升级族
- 使用 `MajorUpgrade` 处理标准升级路径
- 默认阻止 downgrade
- 卸载仅清理 MSI 安装的文件与 PATH 变更
- 卸载不主动删除用户配置或运行数据
