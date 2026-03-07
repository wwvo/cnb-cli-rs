# cnb issue exist

```
cnb issue exist <NUMBER>
```

检查指定编号的 Issue 是否存在。

如果 Issue 存在，输出其标题；如果不存在，输出 `false`。
适合在脚本中用于条件判断。

## 参数

`<NUMBER>`
: Issue 编号（必填）

## 选项

无子命令特有选项。

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

## 示例

```bash
# Issue 存在
$ cnb issue exist 123
修复登录页面样式问题

# Issue 不存在
$ cnb issue exist 99999
false

# 在脚本中使用
$ if [ "$(cnb issue exist 123)" != "false" ]; then echo "Issue 存在"; fi
```

## API

| 步骤       | API                                     | 方法 | 说明            |
| ---------- | --------------------------------------- | ---- | --------------- |
| 获取 Issue | `${API}/repos/{repo}/-/issues/{number}` | GET  | 获取 Issue 详情 |

返回 `404` 时输出 `false`，否则输出 Issue 标题。

## 另请参阅

- [cnb issue](/issue/)
- [cnb issue view](/issue/view)
