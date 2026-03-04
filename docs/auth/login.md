# cnb auth login

```
cnb auth login [flags]
```

登录 CNB 平台。

默认使用交互式隐藏输入方式获取 Token。也可以通过 `--token` 参数直接指定。

登录流程：
1. 获取 Token（参数传入或交互式隐藏输入）
2. 调用 `/user` API 验证 Token 有效性
3. 将 Token 和用户名保存到 `~/.cnb/config.toml`

CNB CLI 也支持通过环境变量传递 Token，适合 CI/CD 等无头环境。详见 [cnb config](/config/)。

## 选项

`--token <TOKEN>`
:   直接指定 Token，不提供则交互式输入

## 示例

```bash
# 交互式登录（隐藏输入）
$ cnb auth login

# 直接指定 Token
$ cnb auth login --token cnb_xxxxxxxxxxxx

# 指定域名登录
$ cnb --domain example.com auth login
```

## 另请参阅

- [cnb auth](/auth/)
