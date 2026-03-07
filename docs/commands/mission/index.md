# cnb mission

```
cnb mission <subcommand>
```

管理组织下的任务集（Mission），包括创建、删除、列表查询、可见性设置，以及视图的配置与排序。

## 可用子命令

### 任务集基础管理

| 子命令          | 说明                 |
|-----------------|----------------------|
| list            | 列出组织下的任务集   |
| create          | 创建任务集           |
| delete          | 删除任务集           |
| set-visibility  | 设置任务集可见性     |

### 视图管理

| 子命令      | 说明             |
|-------------|------------------|
| view list   | 列出视图         |
| view get    | 获取视图配置     |
| view set    | 设置视图配置     |
| view add    | 添加/修改视图    |
| view sort   | 排序视图         |

## 示例

```bash
# 列出组织下的任务集
cnb mission list --group my-org

# 创建任务集
cnb mission create --group my-org --name "Sprint 2025-Q2"

# 删除任务集
cnb mission delete my-org/sprint-2025-q1

# 列出视图
cnb mission view list my-org/sprint-2025-q1
```
