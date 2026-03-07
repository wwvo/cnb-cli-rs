# cnb registry set-visibility

```
cnb registry set-visibility <registry> <visibility>
```

设置制品库可见性。

## 选项

- `<registry>`: 制品库路径，格式 `group/registry`（必填）
- `<visibility>`: 可见性：`public`、`private`、`secret`（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 设为私有
$ cnb registry set-visibility my-org/my-registry private

# 设为公开
$ cnb registry set-visibility my-org/my-registry public
```

## 另请参阅

- [cnb registry](/registry/)
- [cnb registry list](/registry/list)
