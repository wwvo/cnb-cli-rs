# cnb mission view set

```
cnb mission view set <mission> --file <path>
cnb mission view set <mission> --stdin
```

通过 JSON 文件或标准输入设置视图配置。

## 参数

| 参数             | 说明                 |
|------------------|----------------------|
| `<mission>`      | 任务集路径           |
| `--file <path>`  | 从 JSON 文件读取配置 |
| `--stdin`        | 从标准输入读取       |

## 示例

```bash
cnb mission view set my-org/sprint-2025-q1 --file view-config.json
cat view-config.json | cnb mission view set my-org/sprint-2025-q1 --stdin
```
