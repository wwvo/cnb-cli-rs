# cnb badge upload

```
cnb badge upload --key <key> --sha <sha> --message <msg> [options]
```

上传自定义徽章数据。

## 参数

| 参数                | 缩写 | 说明                         |
|---------------------|------|------------------------------|
| `--key <key>`       | `-k` | 徽章 key（如 security/tca）  |
| `--sha <sha>`       |      | Commit ID                    |
| `--message <msg>`   | `-m` | 徽章右侧显示内容             |
| `--value <num>`     |      | 徽章数值                     |
| `--link <url>`      | `-l` | 点击徽章右侧的跳转链接       |
| `--latest`          |      | 同时上传 latest 徽章         |

## 示例

```bash
cnb badge upload --key security/tca --sha abc12345 \
    --message "A+" --value 95 \
    --link "https://example.com/report" \
    --latest
# ✓ 徽章已上传
#   Commit URL: https://cnb.cool/.../badge/git/abc12345/security/tca
#   Latest URL: https://cnb.cool/.../badge/git/latest/security/tca
```

## 另请参阅

- [cnb badge list](/badge/list)
