# cnb issue list

```
cnb issue list [flags]
```

列出仓库所有状态为 `open` 的 Issue。

输出为表格格式，包含 Issue 编号、标题、最后活动时间和不活跃天数。

## 选项

`-d, --stale-days <N>`
: 仅显示超过 N 天没有活动的 Issue（默认: `0`，即不过滤）

## 示例

```bash
# 列出所有 open 状态的 Issue
$ cnb issue list

# 仅显示超过 30 天没有活动的 Issue
$ cnb issue list --stale-days 30

# 简写
$ cnb issue list -d 7
```

## 另请参阅

- [cnb issue](/issue/)
