# cnb badge get

```
cnb badge get <sha> <badge> [options]
```

获取指定徽章数据，默认输出 SVG 格式。

## 参数

| 参数                | 缩写 | 说明                       |
|---------------------|------|----------------------------|
| `<sha>`             |      | Commit SHA 或 `latest`     |
| `<badge>`           |      | 徽章名称                   |
| `--branch <branch>` | `-b` | 指定分支                   |
| `--svg`             |      | 输出 SVG 格式（默认）      |

## 示例

```bash
# 获取最新 CI 状态徽章 JSON 数据
cnb badge get latest ci/status/push --json

# 获取指定 commit 的徽章 SVG
cnb badge get 89d48c07 ci/status/push --svg > badge.svg

# 指定分支
cnb badge get latest ci/status/push --branch main --json
```

## 另请参阅

- [cnb badge list](/badge/list)
