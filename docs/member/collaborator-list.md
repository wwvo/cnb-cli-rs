# cnb member collaborator-list

```
cnb member collaborator-list --group <group> [options]
```

列出组织外部贡献者。

## 参数

| 参数              | 缩写 | 说明         |
|-------------------|------|--------------|
| `--group <group>` | `-g` | 组织路径     |
| `--role <role>`   | `-r` | 按角色过滤   |
| `--search <kw>`   | `-s` | 搜索成员     |

## 示例

```bash
cnb member collaborator-list --group myorg
cnb member collaborator-list --group myorg --json
```
