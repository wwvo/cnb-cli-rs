# cnb badge

```
cnb badge <subcommand>
```

管理仓库的 CI/CD 徽章，包括获取徽章数据、列出徽章和上传自定义徽章。

## 可用子命令

| 子命令  | 说明             |
|---------|------------------|
| get     | 获取指定徽章     |
| list    | 列出仓库徽章     |
| upload  | 上传自定义徽章   |

## 示例

```bash
# 列出仓库徽章
cnb badge list

# 获取最新 CI 状态徽章
cnb badge get latest ci/status/push --json

# 上传自定义徽章
cnb badge upload --key security/tca --sha abc12345 --message "A+"
```

## 另请参阅

- [cnb build](/build/)
