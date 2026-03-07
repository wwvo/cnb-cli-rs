# cnb mission create

```
cnb mission create --group <group> --name <name> [options]
```

创建任务集。

## 参数

| 参数                       | 缩写 | 说明                           |
|----------------------------|------|--------------------------------|
| `--group <group>`          | `-g` | 组织路径                       |
| `--name <name>`            | `-n` | 任务集名称                     |
| `--description <desc>`     | `-d` | 描述                           |
| `--repos <repo1,repo2>`    |      | 关联仓库列表（逗号分隔）      |
| `--visibility <level>`     |      | 可见性（public/private/secret）|

## 示例

```bash
cnb mission create --group my-org --name "Sprint 2025-Q2"
cnb mission create --group my-org --name "Bug Tracking" \
    --description "缺陷跟踪" \
    --repos "my-org/repo-a,my-org/repo-b" \
    --visibility public
```
