# cnb mission view add

```
cnb mission view add <mission> --name <name> --type <type> [--id <id>]
```

添加或修改任务集视图。

## 参数

| 参数             | 说明                     |
|------------------|--------------------------|
| `<mission>`      | 任务集路径               |
| `--name <name>`  | 视图名称                 |
| `--type <type>`  | 视图类型                 |
| `--id <id>`      | 视图 ID（修改已有视图）  |

## 示例

```bash
# 添加新视图
cnb mission view add my-org/sprint-2025-q1 --name "看板视图" --type kanban

# 修改已有视图
cnb mission view add my-org/sprint-2025-q1 --id view-001 --name "重命名看板" --type kanban
```
