# cnb build stop

```
cnb build stop <sn>
```

停止指定构建。

## 选项

- `<sn>`: 构建号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb build stop cnb-1qa-1i3f5ecau
✓ 构建 cnb-1qa-1i3f5ecau 已停止
```

## 另请参阅

- [cnb build](/build/)
- [cnb build start](/build/start)
- [cnb build status](/build/status)
