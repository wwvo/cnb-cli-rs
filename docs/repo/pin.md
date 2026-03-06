# cnb repo pin

```
cnb repo pin <list|set> [flags]
```

管理仓库墙（置顶仓库）。支持查看用户/组织的仓库墙，以及设置组织的仓库墙。

## 子命令

### `cnb repo pin list`

```
cnb repo pin list [<owner>]
```

列出用户或组织的置顶仓库。

不指定 `owner` 时，列出当前认证用户的仓库墙。

**选项：**

`<owner>`
: 用户名或组织名，可选

**继承的全局选项：**

`--json`
: 以 JSON 格式输出

### `cnb repo pin set`

```
cnb repo pin set --group <GROUP> <repos...>
```

设置组织的仓库墙。传入的仓库路径列表将**替换**现有的仓库墙。

::: warning
此操作会替换组织现有的全部置顶仓库，而非追加。
:::

**选项：**

`<repos...>`
: 要置顶的仓库路径列表（必填，可多个）

`--group <GROUP>`, `-g`
: 目标组织（必填）

## 示例

```bash
# 查看当前用户的仓库墙
$ cnb repo pin list

# 查看组织的仓库墙
$ cnb repo pin list my-org

# 查看指定用户的仓库墙
$ cnb repo pin list octocat

# 设置组织仓库墙
$ cnb repo pin set --group my-org my-org/repo1 my-org/repo2 my-org/repo3
✓ 已更新 my-org 的仓库墙（共 3 个仓库）

# JSON 输出
$ cnb repo pin list --json
```

## API

| 场景             | API                                              | 方法 | 说明             |
| ---------------- | ------------------------------------------------ | ---- | ---------------- |
| 获取组织仓库墙   | `${CNB_API_ENDPOINT}/{slug}/-/pinned-repos`      | GET  | 组织置顶仓库     |
| 获取用户仓库墙   | `${CNB_API_ENDPOINT}/users/{username}/pinned-repos` | GET  | 用户置顶仓库  |
| 设置组织仓库墙   | `${CNB_API_ENDPOINT}/{slug}/-/pinned-repos`      | PUT  | 更新组织仓库墙   |

**API 详情**（OpenAPI）：

- [`GetPinnedRepoByGroup`](https://api.cnb.cool/#/operations/GetPinnedRepoByGroup) — 权限：`group-manage:r`
- [`GetPinnedRepoByID`](https://api.cnb.cool/#/operations/GetPinnedRepoByID) — 权限：`account-engage:r`
- [`SetPinnedRepoByGroup`](https://api.cnb.cool/#/operations/SetPinnedRepoByGroup) — 权限：`group-manage:rw`

## 另请参阅

- [cnb repo](/repo/)
- [cnb repo list](/repo/list)
