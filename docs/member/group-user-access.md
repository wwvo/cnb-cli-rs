# cnb member group-user-access

```
cnb member group-user-access <username> --group <group>
```

查看指定成员在组织的权限层级。

## 参数

| 参数              | 缩写 | 说明         |
|-------------------|------|--------------|
| `<username>`      |      | 用户名       |
| `--group <group>` | `-g` | 组织路径     |

## 示例

```bash
cnb member group-user-access zhangsan --group myorg
cnb member group-user-access zhangsan --group myorg --json
```
