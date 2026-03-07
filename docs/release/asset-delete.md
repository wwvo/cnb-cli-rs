# cnb release asset-delete

```
cnb release asset-delete <TAG> --asset <NAME> [options]
```

删除 Release 中的指定附件。需要交互确认。

## 参数

`TAG`
: Tag 名称（必填）

## 选项

`--asset <NAME>`
: 附件名称（必填）

`--confirm`
: 跳过交互确认

## 示例

```bash
# 删除指定附件（需确认）
$ cnb release asset-delete v1.2.0 --asset old-binary.tar.gz

# 跳过确认
$ cnb release asset-delete v1.2.0 --asset old-binary.tar.gz --confirm
```

## API

| 方法 | 端点 |
|------|------|
| DELETE | `/{repo}/-/releases/{release_id}/assets/{asset_id}` |

## 另请参阅

- [cnb release](/release/)
- [cnb release asset-clean](/release/asset-clean)
- [cnb release view](/release/view)
