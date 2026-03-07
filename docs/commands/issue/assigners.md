# cnb issue assigners

```
cnb issue assigners <subcommand>
```

Issue 处理人管理，支持获取和添加处理人。

## 可用命令

### cnb issue assigners get

```
cnb issue assigners get <NUMBER>
```

获取指定 Issue 的处理人列表。

输出为表格格式，包含处理人的用户名和昵称。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

`--json`
: 以 JSON 格式输出

**示例：**

```bash
# 获取处理人列表
$ cnb issue assigners get 123
USERNAME        NICKNAME
zhangsan        张三
lisi            李四

# JSON 格式
$ cnb --json issue assigners get 123
```

**API：**

| 步骤       | API                                               | 方法 | 说明           |
| ---------- | ------------------------------------------------- | ---- | -------------- |
| 获取处理人 | `${API}/repos/{repo}/-/issues/{number}/assignees` | GET  | 获取处理人列表 |

---

### cnb issue assigners add

```
cnb issue assigners add <NUMBER> [flags]
```

为指定 Issue 添加处理人。多个用户名用逗号分隔，自动去重。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

**选项：**

`-a, --assignees <ASSIGNEES>`
: 处理人用户名，多个用逗号分隔（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

**示例：**

```bash
# 添加单个处理人
$ cnb issue assigners add 123 -a zhangsan
✓ Issue #123 处理人已更新

# 添加多个处理人
$ cnb issue assigners add 123 -a "zhangsan,lisi"
✓ Issue #123 处理人已更新
```

**API：**

| 步骤       | API                                               | 方法 | 说明              |
| ---------- | ------------------------------------------------- | ---- | ----------------- |
| 添加处理人 | `${API}/repos/{repo}/-/issues/{number}/assignees` | POST | 添加 Issue 处理人 |

## 另请参阅

- [cnb issue](/issue/)
- [cnb issue edit](/issue/edit)
