# cnb badge list

```
cnb badge list
```

列出仓库所有徽章。

## 输出列

| 列名   | 说明       |
|--------|------------|
| NAME   | 徽章名称   |
| TYPE   | 徽章类型   |
| STATUS | 分组状态   |
| URL    | 徽章 URL   |

## 示例

```bash
cnb badge list
# NAME                       TYPE      STATUS    URL
# ci/status/push             ci        passing   https://cnb.cool/.../badge/...
# ci/status/pull_request     ci        passing   https://cnb.cool/.../badge/...

cnb badge list --json
```

## 另请参阅

- [cnb badge get](/badge/get)
