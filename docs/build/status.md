# cnb build status

```
cnb build status <sn>
```

查询指定构建的状态和流水线状态。

## 参数

| 参数   | 说明   |
|--------|--------|
| `<sn>` | 构建号 |

## 示例

```bash
cnb build status cnb-1qa-1i3f5ecau
# 构建号:   cnb-1qa-1i3f5ecau
# 状态:     success
# 流水线状态:
#   pipeline-1: success
#   pipeline-2: success

cnb build status cnb-1qa-1i3f5ecau --json
```

## 另请参阅

- [cnb build list](/build/list)
- [cnb build stage](/build/stage)
