# cnb issue view

```
cnb issue view <NUMBER> [flags]
```

查看 Issue 详情。

显示 Issue 的完整信息，包括编号、标题、状态、优先级、作者、处理人、标签、
日期信息、评论数以及正文内容。

支持在浏览器中打开 Issue 页面。

## 参数

`<NUMBER>`
: Issue 编号（必填）

## 选项

`-w, --web`
: 在浏览器中打开 Issue 页面，不调用 API 获取详情

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

`--json`
: 以 JSON 格式输出完整 Issue 数据

## 输出字段

表格模式下，按以下字段展示（仅显示非空字段）：

| 字段     | 说明                                   |
| -------- | -------------------------------------- |
| 编号     | Issue 编号（如 `#42`）                 |
| 标题     | Issue 标题                             |
| 状态     | `open` 或 `closed (completed/not_planned)` |
| 优先级   | `-2P` / `-1P` / `P0` ~ `P3`           |
| 作者     | 昵称 (用户名) 或 用户名               |
| 处理人   | 多个处理人逗号分隔                     |
| 标签     | 多个标签逗号分隔                       |
| 创建时间 | ISO 8601 格式                          |
| 更新时间 | ISO 8601 格式                          |
| 开始日期 | `YYYY-MM-DD`                           |
| 结束日期 | `YYYY-MM-DD`                           |
| 评论数   | 评论数量                               |

表格之后输出 Issue 正文（Markdown 格式）。

## 示例

```bash
# 查看 Issue 详情
$ cnb issue view 42
字段          值
编号          #42
标题          修复登录页面样式问题
状态          open
优先级        P1
作者          张三 (zhangsan)
处理人        李四 (lisi), 王五 (wangwu)
标签          bug, frontend
创建时间      2025-01-15T10:30:00Z
评论数        3

页面样式在移动端显示异常...

# 在浏览器中打开
$ cnb issue view 42 --web
ℹ 正在打开 https://cnb.cool/org/repo/-/issues/42

# JSON 格式
$ cnb --json issue view 42
```

## API

| 步骤           | API                                      | 方法 | 说明            |
| -------------- | ---------------------------------------- | ---- | --------------- |
| 获取 Issue     | `${API}/repos/{repo}/-/issues/{number}`  | GET  | 获取 Issue 详情 |

## 另请参阅

- [cnb issue](/issue/)
- [cnb issue edit](/issue/edit)
- [cnb issue list](/issue/list)
