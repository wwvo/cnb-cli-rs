# cnb build list

```
cnb build list [options]
```

列出仓库构建记录，支持多种过滤条件。

## 参数

| 参数                | 缩写 | 说明                                 |
|---------------------|------|--------------------------------------|
| `--status <status>` | `-s` | 按状态过滤（pending/success/error/cancel） |
| `--event <event>`   | `-e` | 按事件过滤                           |
| `--branch <branch>` | `-b` | 按源分支过滤                         |
| `--user <username>` | `-u` | 按用户过滤                           |
| `--since <date>`    |      | 开始日期（YYYY-MM-DD）               |
| `--until <date>`    |      | 结束日期（YYYY-MM-DD）               |
| `--limit <n>`       | `-n` | 每页数量（默认 30）                  |

## 示例

```bash
# 列出最近构建
cnb build list

# 过滤失败的构建
cnb build list --status error

# 按分支和日期过滤
cnb build list --branch main --since 2025-01-01

# JSON 格式
cnb build list --json
```

## 另请参阅

- [cnb build status](/build/status)
- [cnb build start](/build/start)
