# cnb pull update

```
cnb pull update <NUMBER> [flags]
```

更新指定 Pull Request 的标题、描述或状态。

至少需要指定 `--title`、`--body` 或 `--state` 中的一个。

## 参数

`<NUMBER>`
: PR 编号（必填）

## 选项

`-t, --title <TITLE>`
: 修改标题

`-b, --body <BODY>`
: 修改描述

`-s, --state <STATE>`
: 修改状态，可选值：`open`、`closed`

## 示例

```bash
# 修改标题
$ cnb pull update 42 --title "新标题"

# 关闭 PR
$ cnb pull update 42 --state closed

# 同时修改标题和描述
$ cnb pull update 42 -t "新标题" -b "新描述"
```

## 另请参阅

- [cnb pull](/pull/)
