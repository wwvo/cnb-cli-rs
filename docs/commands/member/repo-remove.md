# cnb member repo-remove

```
cnb member repo-remove <username> [options]
```

移除仓库成员。

执行前会要求确认，可通过 `--yes` 跳过确认提示。

## 选项

- `<username>`: 用户名（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 移除仓库成员（需确认）
$ cnb member repo-remove zhangsan
确认移除成员 zhangsan？(y/N) y
✓ 已移除成员 zhangsan

# 跳过确认
$ cnb member repo-remove zhangsan --yes
```

## 另请参阅

- [cnb member](/member/)
- [cnb member repo-list](/member/repo-list)
- [cnb member repo-add](/member/repo-add)
