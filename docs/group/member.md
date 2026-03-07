# cnb group member

组织成员管理。包含 list、add、update、remove、access-level 五个子命令。

## 子命令

### member list

```
cnb group member list <GROUP> [options]
```

列出组织的直接成员。

**参数：**

`GROUP`
: 组织路径（必填）

**选项：**

`-r, --role <ROLE>`
: 按角色过滤

`-s, --search <KEYWORD>`
: 关键字搜索

`--inherited`
: 列出继承成员（来自父组织）

**输出列：**

| 列       | 说明           |
| -------- | -------------- |
| 用户名   | 成员用户名     |
| 昵称     | 成员昵称       |
| 权限     | 权限级别       |
| 加入时间 | 加入组织的时间 |

---

### member add

```
cnb group member add <GROUP> <USERNAME> [options]
```

添加组织成员。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 要添加的用户名（必填）

**选项：**

`-r, --role <ROLE>`
: 权限级别（默认：Developer）。可选值：Guest/Reporter/Developer/Master/Owner

---

### member update

```
cnb group member update <GROUP> <USERNAME> --role <ROLE>
```

更新成员权限。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 用户名（必填）

**选项：**

`-r, --role <ROLE>`
: 新的权限级别（必填）。可选值：Guest/Reporter/Developer/Master/Owner

---

### member remove

```
cnb group member remove <GROUP> <USERNAME> [options]
```

移除组织成员。需要交互确认。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 要移除的用户名（必填）

**选项：**

`--confirm`
: 跳过交互确认

---

### member access-level

```
cnb group member access-level <GROUP> [USERNAME]
```

查看成员权限级别。不指定用户名则查看当前用户权限。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 用户名（可选，不指定则查看当前用户权限）

## 示例

```bash
# 列出成员
$ cnb group member list my-org

# 按角色过滤
$ cnb group member list my-org --role Owner

# 列出继承成员
$ cnb group member list my-org/sub-team --inherited

# 添加成员
$ cnb group member add my-org alice --role Developer

# 更新权限
$ cnb group member update my-org alice --role Master

# 移除成员
$ cnb group member remove my-org alice

# 查看我的权限
$ cnb group member access-level my-org

# 查看指定用户权限层级
$ cnb group member access-level my-org alice

# JSON 输出
$ cnb group member list my-org --json
```

## API

| 操作         | 方法   | 端点                                         |
| ------------ | ------ | -------------------------------------------- |
| 列出成员     | GET    | `/{group}/-/members`                         |
| 列出继承成员 | GET    | `/{group}/-/inherit-members`                 |
| 添加成员     | POST   | `/{group}/-/members/{username}`              |
| 更新成员     | PUT    | `/{group}/-/members/{username}`              |
| 移除成员     | DELETE | `/{group}/-/members/{username}`              |
| 当前用户权限 | GET    | `/{group}/-/members/access-level`            |
| 指定用户权限 | GET    | `/{group}/-/members/{username}/access-level` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
