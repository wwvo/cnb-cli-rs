# cnb label issue-remove

```
cnb label issue-remove <number> <name>
```

移除指定 Issue 的单个标签。

## 选项

- `<number>`: Issue 编号（必填）
- `<name>`: 标签名称（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb label issue-remove 42 "bug"
✓ 已从 Issue #42 移除标签: bug
```

## 另请参阅

- [cnb label](/label/)
- [cnb label issue-list](/label/issue-list)
- [cnb label issue-clear](/label/issue-clear)
