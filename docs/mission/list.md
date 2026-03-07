# cnb mission list

```
cnb mission list --group <group> [options]
```

列出组织下的任务集。

## 参数

| 参数                 | 缩写 | 说明         |
|----------------------|------|--------------|
| `--group <group>`    | `-g` | 组织路径     |
| `--search <keyword>` | `-s` | 搜索关键字   |
| `--order-by <field>` |      | 排序字段     |
| `--desc`             |      | 降序排列     |

## 示例

```bash
cnb mission list --group my-org
cnb mission list --group my-org --search sprint
cnb mission list --group my-org --order-by created_at --desc
cnb mission list --group my-org --json
```
