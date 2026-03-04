# cnb workspace closed-clean

```
cnb workspace closed-clean
```

清理所有已关闭的云原生工作区。

自动分页获取所有状态为 `closed` 的工作区，逐个执行删除操作。每个工作区的清理结果会实时输出，包含 slug 和 pipelineId 信息。

## 示例

```bash
$ cnb workspace closed-clean
共找到 3 个已关闭的工作区

[INFO] 开始清理工作区 slug=my-org/my-repo pipelineId=abc123
[SUCCESS] 已清理工作区 slug=my-org/my-repo pipelineId=abc123
```

## 另请参阅

- [cnb workspace](/workspace/)
