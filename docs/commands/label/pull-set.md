# cnb label pull-set

```
cnb label pull-set <number> -l <labels>
```

替换指定 Pull Request 的所有标签（现有标签将被完全替换）。

## 选项

- `<number>`: Pull 编号（必填）
- `-l, --labels <LABELS>`: 新标签列表，逗号分隔或多次指定（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb label pull-set 10 -l "wip"
✓ 已替换 Pull #10 的标签为: wip
```

## 另请参阅

- [cnb label](/label/)
- [cnb label pull-list](/label/pull-list)
- [cnb label pull-clear](/label/pull-clear)
