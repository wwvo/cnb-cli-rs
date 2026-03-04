# cnb auth logout

```
cnb auth logout
```

退出登录，移除当前域名的认证信息。

根据 Token 来源执行不同操作：

- **配置文件** — 自动从 `~/.cnb/config.toml` 中移除对应域名的 Token 和用户名
- **域名专属环境变量**（`CNB_TOKEN_{DOMAIN}`）— 无法通过 CLI 移除，提示用户手动清除
- **通用环境变量**（`CNB_TOKEN`）— 无法通过 CLI 移除，提示用户手动清除
- **未登录** — 提示当前未登录

## 输出示例

配置文件登录时：

```
✓ 已退出 (cnb.cool)
```

环境变量登录时：

```
Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除
请手动执行: $env:CNB_TOKEN=""
```

## 另请参阅

- [cnb auth](/auth/)
