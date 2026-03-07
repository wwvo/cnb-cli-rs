# cnb label pull-list

```
cnb label pull-list <number>
```

列出指定 Pull Request 的所有标签。

## 选项

- `<number>`: Pull 编号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb label pull-list 10
feature
review-needed

# JSON 格式输出
$ cnb label pull-list 10 --json
```

## 另请参阅

- [cnb label](/label/)
- [cnb label pull-add](/label/pull-add)
- [cnb label pull-remove](/label/pull-remove)
