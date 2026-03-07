# cnb member group-inherited

```
cnb member group-inherited --group <group> [options]
```

列出组织继承成员。

## 参数

| 参数              | 缩写 | 说明         |
|-------------------|------|--------------|
| `--group <group>` | `-g` | 组织路径     |
| `--role <role>`   | `-r` | 按角色过滤   |
| `--search <kw>`   | `-s` | 搜索成员     |

## 示例

```bash
cnb member group-inherited --group myorg
cnb member group-inherited --group myorg --role Master
cnb member group-inherited --group myorg --json
```
