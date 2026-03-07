# cnb workspace

云原生工作区管理，包括启动、停止、删除、查看和清理工作区等操作。

## 可用命令

| 命令                                    | 说明               |
| --------------------------------------- | ------------------ |
| [list](/workspace/list)                 | 列出我的工作区     |
| [start](/workspace/start)               | 启动工作区         |
| [stop](/workspace/stop)                 | 停止工作区         |
| [delete](/workspace/delete)             | 删除工作区         |
| [detail](/workspace/detail)             | 查看工作区详情     |
| [closed-clean](/workspace/closed-clean) | 清理已关闭的工作区 |

## 全局选项

`--json`
: 以 JSON 格式输出（适用于 list、start、stop、detail 等命令）

`--repo <REPO>`
: 指定仓库路径（适用于 start、detail 等命令）

## 示例

```bash
# 列出所有工作区
$ cnb workspace list

# 启动当前仓库的开发环境
$ cnb workspace start

# 停止工作区
$ cnb workspace stop -p <pipeline-id>

# 查看工作区详情
$ cnb workspace detail --sn <sn>

# 清理已关闭的工作区
$ cnb workspace closed-clean
```

## 另请参阅

- [cnb](/guide/cnb)
