# cnb issue comment

```
cnb issue comment <NUMBER> [flags]
```

为指定 Issue 创建评论。

评论内容通过 `--comment` 参数传入，支持 Markdown 格式。

## 参数

`<NUMBER>`
: Issue 编号（必填）

## 选项

`-c, --comment <COMMENT>`
: 评论内容（必填），支持 Markdown 格式

**继承的全局选项：**

`--repo <REPO>`
: 指定仓库路径

## 示例

```bash
# 创建评论
$ cnb issue comment 123 -c "已确认问题，正在修复"
✓ Issue #123 评论已创建

# 使用 Markdown 格式
$ cnb issue comment 42 -c "## 分析结果\n- 原因：配置错误\n- 影响范围：登录模块"
```

## API

| 步骤     | API                                              | 方法 | 说明            |
| -------- | ------------------------------------------------ | ---- | --------------- |
| 创建评论 | `${API}/repos/{repo}/-/issues/{number}/comments` | POST | 创建 Issue 评论 |

**请求体：**

```json
{
  "body": "评论内容"
}
```

## 另请参阅

- [cnb issue](/issue/)
- [cnb issue view](/issue/view)
