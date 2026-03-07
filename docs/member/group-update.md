# cnb member group-update

```
cnb member group-update <username> --group <group> --role <role>
```

更新组织成员权限。

## 参数

| 参数              | 缩写 | 说明             |
|-------------------|------|------------------|
| `<username>`      |      | 用户名           |
| `--group <group>` | `-g` | 组织路径         |
| `--role <role>`   | `-r` | 新权限等级       |

## 示例

```bash
cnb member group-update zhangsan --group myorg --role Master
```
