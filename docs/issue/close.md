# cnb issue close

```
cnb issue close [flags]
```

关闭指定编号的 Issue。

将 Issue 状态设置为 `closed`，关闭原因为 `not_planned`。

## 选项

`-n, --number <NUMBER>`
: Issue 编号（必填）

## 示例

```bash
$ cnb issue close --number 123
Issue #123 已关闭
```

## 另请参阅

- [cnb issue](/issue/)
