# cnb workspace delete

```
cnb workspace delete [options]
```

删除指定的云原生开发环境。

## 选项

`-p, --pipeline-id <ID>`
: 流水线 ID（必填）

`--sn <SN>`
: 流水线构建号（暂未支持，请使用 `--pipeline-id`）

## 示例

```bash
# 删除指定工作区
$ cnb workspace delete -p abc123
```

## API

| 方法 | 端点 |
|------|------|
| DELETE | `/user/workspaces/{pipeline_id}` |

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace closed-clean](/workspace/closed-clean)
