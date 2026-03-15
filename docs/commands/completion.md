# cnb-rs completion

```
cnb-rs completion <shell>
```

生成指定 Shell 的命令行补全脚本并输出到 stdout。

支持的 Shell 类型：`bash`、`zsh`、`fish`、`powershell`、`elvish`。

> [!WARNING]
> 改名后补全文件名也随之变化。例如 Fish 的输出文件应从 `cnb.fish` 改为 `cnb-rs.fish`。升级旧版本时请重新生成补全脚本。

## 示例

::: code-group

```bash [Bash]
$ cnb-rs completion bash >> ~/.bashrc
```

```bash [Zsh]
$ cnb-rs completion zsh >> ~/.zshrc
```

```bash [Fish]
$ cnb-rs completion fish > ~/.config/fish/completions/cnb-rs.fish
```

```powershell [PowerShell]
$ cnb-rs completion powershell >> $PROFILE
```

:::

## 另请参阅

- [cnb-rs](/guide/cnb)
- [从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)
