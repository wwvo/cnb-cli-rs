# cnb issue create

```
cnb issue create [flags]
```

创建 Issue。

创建成功后输出 Issue 的 Web 链接。

## 选项

`-t, --title <TITLE>`
: Issue 标题（必填）

`-b, --body <BODY>`
: Issue 描述（默认: 空）

## 示例

```bash
# 创建 Issue
$ cnb issue create --title "修复登录问题"

# 创建带描述的 Issue
$ cnb issue create -t "新功能需求" -b "详细描述..."
```

## 另请参阅

- [cnb issue](/issue/)
