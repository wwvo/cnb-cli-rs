# cnb pull merge

```
cnb pull merge <NUMBER> [flags]
```

合并指定的 Pull Request。

## 参数

`<NUMBER>`
: PR 编号（必填）

## 选项

`-t, --commit-title <TITLE>`
: 合并提交标题（必填）

`-m, --commit-message <MESSAGE>`
: 合并提交信息（默认: 空）

`-s, --merge-style <STYLE>`
: 合并方式，可选值：`merge`、`squash`、`rebase`（默认: `merge`）

## 示例

```bash
# 默认 merge 方式合并
$ cnb pull merge 42 --commit-title "Merge PR #42"

# squash 合并
$ cnb pull merge 42 -t "feat: 新功能" -s squash

# rebase 合并，附带提交信息
$ cnb pull merge 42 -t "fix: 修复问题" -m "详细说明" -s rebase
```

## 另请参阅

- [cnb pull](/pull/)
