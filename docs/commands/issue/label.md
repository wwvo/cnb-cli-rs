# cnb issue label

```
cnb issue label <subcommand>
```

Issue 标签管理，支持列出、添加、替换、删除和清空标签。

标签用于对 Issue 进行分类和标记，便于过滤和管理。

## 可用命令

### cnb issue label list

```
cnb issue label list <NUMBER>
```

列出指定 Issue 的所有标签。

输出为表格格式，包含标签名称、颜色和描述。

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
# 列出标签
$ cnb issue label list 42
名称                 颜色       描述
bug                  #d73a4a    Something isn't working
enhancement          #a2eeef    New feature or request

# 没有标签时
$ cnb issue label list 99
ℹ Issue #99 没有标签

# JSON 格式
$ cnb --json issue label list 42
```

**API：**

| 步骤       | API                                            | 方法 | 说明           |
| ---------- | ---------------------------------------------- | ---- | -------------- |
| 列出标签   | `${API}/repos/{repo}/-/issues/{number}/labels` | GET  | 获取标签列表   |

---

### cnb issue label add

```
cnb issue label add <NUMBER> <LABELS>
```

为指定 Issue 添加标签。不影响已有标签，自动去重去空。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

`<LABELS>`
: 标签名称，多个用逗号分隔（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

**示例：**

```bash
# 添加单个标签
$ cnb issue label add 42 bug
✓ Issue #42 标签已添加

# 添加多个标签
$ cnb issue label add 42 "bug,enhancement,P0"
✓ Issue #42 标签已添加
```

**API：**

| 步骤       | API                                            | 方法 | 说明           |
| ---------- | ---------------------------------------------- | ---- | -------------- |
| 添加标签   | `${API}/repos/{repo}/-/issues/{number}/labels` | POST | 添加标签       |

---

### cnb issue label set

```
cnb issue label set <NUMBER> <LABELS>
```

替换指定 Issue 的所有标签。会移除所有现有标签，设置为新的标签列表。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

`<LABELS>`
: 新的标签列表，多个用逗号分隔（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

**示例：**

```bash
# 替换所有标签
$ cnb issue label set 42 "bug,P1"
✓ Issue #42 标签已替换
```

**API：**

| 步骤       | API                                            | 方法 | 说明                 |
| ---------- | ---------------------------------------------- | ---- | -------------------- |
| 替换标签   | `${API}/repos/{repo}/-/issues/{number}/labels` | PUT  | 替换所有标签         |

---

### cnb issue label remove

```
cnb issue label remove <NUMBER> <NAME>
```

删除指定 Issue 的某个标签。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

`<NAME>`
: 要删除的标签名称（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

**示例：**

```bash
$ cnb issue label remove 42 bug
✓ Issue #42 标签 'bug' 已删除
```

**API：**

| 步骤       | API                                                   | 方法   | 说明           |
| ---------- | ----------------------------------------------------- | ------ | -------------- |
| 删除标签   | `${API}/repos/{repo}/-/issues/{number}/labels/{name}` | DELETE | 删除指定标签   |

---

### cnb issue label clear

```
cnb issue label clear <NUMBER>
```

清空指定 Issue 的所有标签。

**参数：**

`<NUMBER>`
: Issue 编号（必填）

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

**示例：**

```bash
$ cnb issue label clear 42
✓ Issue #42 所有标签已清空
```

**API：**

| 步骤       | API                                            | 方法   | 说明           |
| ---------- | ---------------------------------------------- | ------ | -------------- |
| 清空标签   | `${API}/repos/{repo}/-/issues/{number}/labels` | DELETE | 清空所有标签   |

## 另请参阅

- [cnb issue](/issue/)
- [cnb issue create](/issue/create)
- [cnb issue list](/issue/list)
