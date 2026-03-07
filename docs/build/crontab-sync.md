# cnb build crontab-sync

```
cnb build crontab-sync [branch]
```

同步指定分支的定时任务配置。

## 参数

| 参数       | 说明                 |
|------------|----------------------|
| `[branch]` | 分支名（默认 main） |

## 示例

```bash
# 同步默认分支的定时任务
cnb build crontab-sync
# ✓ 分支 main 的定时任务已同步

# 指定分支
cnb build crontab-sync develop
# ✓ 分支 develop 的定时任务已同步
```

## 另请参阅

- [cnb build start](/build/start)
