# cnb issue assigners

Issue 处理人管理，支持获取和添加操作。

## 可用命令

### cnb issue assigners get

```
cnb issue assigners get [flags]
```

获取指定 Issue 的处理人列表。

**选项：**

`-n, --number <NUMBER>`
: Issue 编号（必填）

**示例：**

```bash
$ cnb issue assigners get --number 123
zhangsan
lisi
```

### cnb issue assigners add

```
cnb issue assigners add [flags]
```

为指定 Issue 添加处理人。多个用户名用逗号分隔，自动去重。

**选项：**

`-n, --number <NUMBER>`
: Issue 编号（必填）

`-a, --assignees <ASSIGNEES>`
: 处理人用户名，多个用逗号分隔（必填）

**示例：**

```bash
$ cnb issue assigners add --number 123 --assignees "zhangsan,lisi"
Issue #123 处理人已更新
```

## 另请参阅

- [cnb issue](/issue/)
