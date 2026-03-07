# cnb member group-remove

```
cnb member group-remove <username> --group <group> [options]
```

移除组织成员。

## 参数

| 参数              | 缩写 | 说明             |
|-------------------|------|------------------|
| `<username>`      |      | 用户名           |
| `--group <group>` | `-g` | 组织路径         |
| `--yes`           | `-y` | 跳过确认提示     |

## 示例

```bash
cnb member group-remove zhangsan --group myorg
cnb member group-remove zhangsan --group myorg --yes
```
