# cnb build

```
cnb build <subcommand>
```

管理仓库的 CI/CD 构建流水线，包括触发构建、停止构建、查询状态、查看日志等。

## 可用子命令

| 子命令       | 说明             |
|--------------|------------------|
| start        | 触发构建         |
| stop         | 停止构建         |
| status       | 查询构建状态     |
| list         | 列出构建记录     |
| stage        | 查看 Stage 详情  |
| download-log | 下载 Runner 日志 |
| delete-log   | 删除构建日志     |
| crontab-sync | 同步定时任务     |

## 示例

```bash
# 触发默认分支构建
cnb build start

# 查看构建列表
cnb build list

# 查询构建状态
cnb build status cnb-1qa-1i3f5ecau

# 停止构建
cnb build stop cnb-1qa-1i3f5ecau
```

## 另请参阅

- [cnb repo](/repo/)
