# cnb build download-log

```
cnb build download-log <pipeline_id> [-o <path>]
```

下载指定流水线的 Runner 日志。

## 参数

| 参数              | 缩写 | 说明                          |
|-------------------|------|-------------------------------|
| `<pipeline_id>`   |      | 流水线 ID                     |
| `--output <path>` | `-o` | 输出文件路径（不指定则 stdout）|

## 示例

```bash
# 输出到终端
cnb build download-log pipeline-1

# 保存到文件
cnb build download-log pipeline-1 -o ./build.log
# ✓ 日志已保存到 ./build.log
```

## 另请参阅

- [cnb build stage](/build/stage)
- [cnb build delete-log](/build/delete-log)
