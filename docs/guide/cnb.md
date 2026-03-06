# cnb

在命令行中高效管理你的 CNB 平台资源。

## 常用命令

- [cnb auth](/auth/) — 认证管理
- [cnb chat](/chat) — 使用自然语言与 CNB OpenAPI 交互
- [cnb config](/config/) — 配置管理
- [cnb issue](/issue/) — Issue 管理
- [cnb pull](/pull/) — Pull Request 管理
- [cnb release](/release/) — Release 管理
- [cnb commit](/commit/) — Commit 管理

## 仓库命令

- [cnb repo](/repo/) — 仓库管理
- [cnb download](/download) — 下载仓库文件
- [cnb info](/info) — 显示仓库和用户信息
- [cnb stats](/stats) — 提交统计仪表盘
- [cnb stars](/stars) — Star 趋势图

## 其他命令

- [cnb completion](/completion) — 生成命令行补全脚本
- [cnb group](/group/) — 组织管理
- [cnb knowledge](/knowledge/) — 知识库管理
- [cnb workspace](/workspace/) — 云原生工作区管理
- [cnb version](/version) — 显示版本信息

## 选项

```bash
--domain <DOMAIN>     指定 CNB 域名（默认: cnb.cool）
--repo <OWNER/REPO>   指定仓库路径
--version             显示版本号
-h, --help            显示帮助信息
```

## 示例

```bash
$ cnb auth login
$ cnb issue list
$ cnb chat --do "查看我的仓库列表"
$ cnb download --files README.md
$ cnb pull create
```
