# cnb member repo-all

```
cnb member repo-all [options]
```

列出仓库所有有效成员（含继承）。

## 参数

| 参数               | 缩写 | 说明                     |
|--------------------|------|--------------------------|
| `--role <role>`    | `-r` | 按角色过滤               |
| `--search <kw>`    | `-s` | 搜索成员                 |
| `--names <names>`  |      | 精准匹配用户名（逗号分隔）|
| `--order-by <col>` |      | 排序字段                 |
| `--desc`           |      | 降序排列                 |

## 示例

```bash
cnb member repo-all
cnb member repo-all --role Developer --desc
cnb member repo-all --names "zhangsan,lisi"
cnb member repo-all --json
```
