# cnb build stage

```
cnb build stage <sn> <pipeline_id> <stage_id>
```

查看指定构建的 Stage 详情，包括状态、耗时和日志内容。

## 参数

| 参数            | 说明      |
|-----------------|-----------|
| `<sn>`          | 构建号    |
| `<pipeline_id>` | 流水线 ID |
| `<stage_id>`    | Stage ID  |

## 示例

```bash
cnb build stage cnb-1qa-1i3f5ecau pipeline-1 stage-build
# Stage:    stage-build
# 状态:     success
# 耗时:     45s
# 日志:
#   Step 1/5: Pulling image...
#   Step 2/5: Building...
```

## 另请参阅

- [cnb build status](/build/status)
- [cnb build download-log](/build/download-log)
