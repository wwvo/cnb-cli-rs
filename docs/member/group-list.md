# cnb member group-list

```
cnb member group-list --group <group> [options]
```

列出组织直接成员。

## 参数

| 参数              | 缩写 | 说明         |
|-------------------|------|--------------|
| `--group <group>` | `-g` | 组织路径     |
| `--role <role>`   | `-r` | 按角色过滤   |
| `--search <kw>`   | `-s` | 搜索成员     |

## 示例

```bash
cnb member group-list --group myorg
cnb member group-list --group myorg --role Owner
cnb member group-list --group myorg --json
```
