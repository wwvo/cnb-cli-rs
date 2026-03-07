# cnb member repo-add

```
cnb member repo-add <username> --role <role> [options]
```

添加仓库成员。

## 参数

| 参数                      | 缩写 | 说明                       |
|---------------------------|------|----------------------------|
| `<username>`              |      | 用户名                     |
| `--role <role>`           | `-r` | 权限等级                   |
| `--outside-collaborator`  |      | 标记为外部贡献者           |

## 示例

```bash
cnb member repo-add zhangsan --role Developer
cnb member repo-add lisi --role Reporter --outside-collaborator
```
