# cnb mission view sort

```
cnb mission view sort <mission> --ids <id1,id2,id3>
```

排序任务集视图。

## 参数

| 参数                   | 说明                               |
|------------------------|------------------------------------|
| `<mission>`            | 任务集路径                         |
| `--ids <id1,id2,id3>`  | 视图 ID 列表（按期望顺序，逗号分隔）|

## 示例

```bash
cnb mission view sort my-org/sprint-2025-q1 --ids "view-002,view-001,view-003"
```
