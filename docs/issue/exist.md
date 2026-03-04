# cnb issue exist

```
cnb issue exist [flags]
```

检查指定编号的 Issue 是否存在。

如果 Issue 存在，输出其标题；如果不存在，输出 `false`。

## 选项

`-n, --number <NUMBER>`
: Issue 编号（必填）

## 示例

```bash
# Issue 存在
$ cnb issue exist --number 123
修复登录页面样式问题

# Issue 不存在
$ cnb issue exist --number 99999
false
```

## 另请参阅

- [cnb issue](/issue/)
