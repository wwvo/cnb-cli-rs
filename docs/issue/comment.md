# cnb issue comment

```
cnb issue comment [flags]
```

为指定 Issue 创建评论。

## 选项

`-n, --number <NUMBER>`
: Issue 编号（必填）

`-c, --comment <COMMENT>`
: 评论内容（必填）

## 示例

```bash
$ cnb issue comment --number 123 --comment "已确认问题，正在修复"
Issue #123 评论已创建

# 简写
$ cnb issue comment -n 123 -c "LGTM"
```

## 另请参阅

- [cnb issue](/issue/)
