# cnb build start

```
cnb build start [options]
```

触发构建。默认触发主分支构建。

## 参数

| 参数                    | 缩写 | 说明                          |
|-------------------------|------|-------------------------------|
| `--branch <branch>`     | `-b` | 触发分支                      |
| `--tag <tag>`           | `-t` | 触发 tag（优先级高于 branch） |
| `--sha <sha>`           |      | Commit ID（优先级高于 tag）   |
| `--event <event>`       | `-e` | 事件名（默认 api_trigger）    |
| `--config <yaml>`       |      | 配置文件内容（YAML）          |
| `--env <key=value>`     |      | 环境变量（可多次指定）        |
| `--sync`                |      | 等待构建正式触发再返回        |

## 触发优先级

```
sha > tag > branch > 默认主分支
```

## 示例

```bash
# 触发默认分支构建
cnb build start

# 指定分支和事件
cnb build start -b develop -e "api_trigger_deploy"

# 指定 tag 构建
cnb build start -t v1.0.0

# 传入环境变量
cnb build start --env "DEPLOY_ENV=production" --env "VERSION=1.0.0"

# 同步等待触发
cnb build start --sync
```

## 另请参阅

- [cnb build stop](/build/stop)
- [cnb build status](/build/status)
