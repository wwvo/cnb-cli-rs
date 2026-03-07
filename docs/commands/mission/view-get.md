# cnb mission view get

```
cnb mission view get <mission> --id <view_id>
```

获取视图配置详情。

返回视图的完整配置信息，包括名称、类型、筛选条件、排序规则等。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `--id <VIEW_ID>`: 视图 ID（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb mission view get my-org/sprint-2025-q1 --id view-001

# JSON 格式输出
$ cnb mission view get my-org/sprint-2025-q1 --id view-001 --json
```

## 另请参阅

- [cnb mission](/mission/)
- [cnb mission view list](/mission/view-list)
- [cnb mission view set](/mission/view-set)
