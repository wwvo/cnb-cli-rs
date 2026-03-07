# cnb workspace start

```
cnb workspace start [options]
```

启动云原生开发环境。如果已有环境则直接打开，否则创建新的开发环境。

## 选项

`-b, --branch <BRANCH>`
: 分支名（默认从当前 Git 分支推断，未检测到则使用 main）

`-t, --tag <TAG>`
: Tag 名称

`--open`
: 自动在浏览器中打开

## 示例

```bash
# 启动当前仓库的开发环境
$ cnb workspace start

# 指定分支
$ cnb workspace start -b feature/new-ui

# 指定 Tag
$ cnb workspace start -t v1.0.0

# 启动并自动打开浏览器
$ cnb workspace start --open

# JSON 输出
$ cnb workspace start --json
```

## API

| 方法 | 端点 |
|------|------|
| POST | `/{repo}/-/workspace/start` |

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace stop](/workspace/stop)
- [cnb workspace detail](/workspace/detail)
