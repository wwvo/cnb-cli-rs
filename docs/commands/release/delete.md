# cnb release delete

```
cnb release delete <TAG> [options]
```

删除指定 Release。此操作不可逆，需要交互确认。

## 参数

`TAG`
: Tag 名称（必填）

## 选项

`--confirm`
: 跳过交互确认

## 示例

```bash
# 删除 Release（需确认）
$ cnb release delete v1.0.0-beta

# 跳过确认直接删除
$ cnb release delete v1.0.0-beta --confirm
```

## API

| 方法 | 端点 |
|------|------|
| DELETE | `/{repo}/-/releases/{release_id}` |

## 另请参阅

- [cnb release](/release/)
- [cnb release list](/release/list)
