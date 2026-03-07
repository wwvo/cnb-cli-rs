# cnb member collaborator-remove

```
cnb member collaborator-remove <username> --group <group> [options]
```

移除外部贡献者。

## 参数

| 参数              | 缩写 | 说明             |
|-------------------|------|------------------|
| `<username>`      |      | 用户名           |
| `--group <group>` | `-g` | 组织路径         |
| `--yes`           | `-y` | 跳过确认提示     |

## 示例

```bash
cnb member collaborator-remove zhangsan --group myorg
cnb member collaborator-remove zhangsan --group myorg --yes
```
