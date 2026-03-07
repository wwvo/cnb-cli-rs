# cnb user activities

```
cnb user activities [<username>] [options]
```

查看用户在指定时间段内的活动汇总。

按仓库分组输出 commit、PR、Issue、Review 等活动的统计数据。
不指定日期默认查看当月数据，支持按月（`yyyyMM`）或按日（`yyyyMMdd`）查询。

## 选项

- `[<username>]`: 用户名（不指定则查看当前用户）
- `-d, --date <DATE>`: 查询日期，格式 `yyyyMM`（按月）或 `yyyyMMdd`（按日）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当月活动汇总
$ cnb user activities

# 查看指定月份
$ cnb user activities --date 202412

# 查看指定日期
$ cnb user activities --date 20250115

# 查看其他用户
$ cnb user activities zhangsan --date 202501

# JSON 格式输出
$ cnb user activities --json
```

## 另请参阅

- [cnb user](/user/)
- [cnb user activity-detail](/user/activity-detail)
