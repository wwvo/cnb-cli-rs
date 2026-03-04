# cnb issue download

```
cnb issue download [flags]
```

下载 Issue 为 Markdown 文件，保存到 `.issues/` 目录。

每个 Issue 生成一个 `{编号}.md` 文件，包含标题、描述和所有评论。必须指定 `--number` 或 `--all` 之一。

## 选项

`-n, --number <NUMBER>`
: 指定下载的 Issue 编号

`--all`
: 下载所有 Issue（包括 `open` 和 `closed` 状态）

## 示例

```bash
# 下载单个 Issue
$ cnb issue download --number 123
已下载 Issue #123: 修复登录问题
下载完成，共下载了 1 个 Issue

# 下载所有 Issue
$ cnb issue download --all
```

## 另请参阅

- [cnb issue](/issue/)
