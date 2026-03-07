# cnb workspace detail

```
cnb workspace detail --sn <SN> [options]
```

查看工作区详情，获取各种 IDE 的访问地址。

## 选项

`--sn <SN>`
: 流水线构建号（必填）

## 输出示例

```
WebIDE:     https://cnb.cool/workspace/webide/...
VSCode:     vscode://vscode-remote/ssh-remote+...
Cursor:     cursor://vscode-remote/ssh-remote+...
CodeBuddy:  codebuddy://vscode-remote/ssh-remote+...
SSH:        ssh -o StrictHostKeyChecking=no ...
```

## 示例

```bash
# 查看工作区详情
$ cnb workspace detail --sn 20250115-001

# JSON 输出
$ cnb workspace detail --sn 20250115-001 --json
```

## API

| 方法 | 端点 |
|------|------|
| GET | `/{repo}/-/workspace/detail/{sn}` |

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace start](/workspace/start)
