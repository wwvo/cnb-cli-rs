# cnb member

```
cnb member <subcommand>
```

管理仓库成员、组织成员和外部贡献者。

## 可用子命令

### 仓库成员

| 子命令              | 说明                           |
|---------------------|--------------------------------|
| repo-list           | 列出仓库直接成员               |
| repo-add            | 添加仓库成员                   |
| repo-update         | 更新仓库成员权限               |
| repo-remove         | 移除仓库成员                   |
| repo-access-level   | 查看自己在仓库的权限           |
| repo-user-access    | 查看指定成员在仓库的权限层级   |
| repo-inherited      | 列出仓库继承成员               |
| repo-all            | 列出仓库所有有效成员           |

### 组织成员

| 子命令              | 说明                           |
|---------------------|--------------------------------|
| group-list          | 列出组织直接成员               |
| group-add           | 添加组织成员                   |
| group-update        | 更新组织成员权限               |
| group-remove        | 移除组织成员                   |
| group-access-level  | 查看自己在组织的权限           |
| group-user-access   | 查看指定成员在组织的权限层级   |
| group-inherited     | 列出组织继承成员               |

### 外部贡献者

| 子命令                | 说明                 |
|-----------------------|----------------------|
| collaborator-list     | 列出外部贡献者       |
| collaborator-update   | 更新外部贡献者权限   |
| collaborator-remove   | 移除外部贡献者       |

## 权限等级

| 等级      | 说明         |
|-----------|--------------|
| Guest     | 访客         |
| Reporter  | 报告者       |
| Developer | 开发者       |
| Master    | 管理者       |
| Owner     | 拥有者       |

## 示例

```bash
# 列出仓库成员
cnb member repo-list

# 添加仓库成员
cnb member repo-add zhangsan --role Developer

# 列出组织成员
cnb member group-list --group myorg

# 列出外部贡献者
cnb member collaborator-list --group myorg
```
