# cnb member group-access-level

```
cnb member group-access-level --group <group> [options]
```

查看自己在组织的权限。

## 参数

| 参数                | 缩写 | 说明             |
|---------------------|------|------------------|
| `--group <group>`   | `-g` | 组织路径         |
| `--include-inherit` |      | 包含继承权限     |

## 示例

```bash
cnb member group-access-level --group myorg
cnb member group-access-level --group myorg --json
```
