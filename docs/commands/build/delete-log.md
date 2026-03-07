# cnb build delete-log

```
cnb build delete-log <sn> [-y]
```

删除指定构建的日志。默认需要交互确认。

## 参数

| 参数    | 缩写 | 说明           |
|---------|------|----------------|
| `<sn>`  |      | 构建号         |
| `--yes` | `-y` | 跳过确认提示   |

## 示例

```bash
cnb build delete-log cnb-1qa-1i3f5ecau
# ⚠ 确认删除构建 cnb-1qa-1i3f5ecau 的日志？(y/N) y
# ✓ 构建日志已删除

cnb build delete-log cnb-1qa-1i3f5ecau -y
# ✓ 构建日志已删除
```

## 另请参阅

- [cnb build list](/build/list)
- [cnb build download-log](/build/download-log)
