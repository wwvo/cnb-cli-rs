# cnb release

Release 版本管理，包括查看、创建、更新、删除 Release，以及附件的上传、下载、统计和清理等操作。

## 可用命令

- [cnb release list](/release/list) — 列出 Release
- [cnb release view](/release/view) — 查看 Release 详情
- [cnb release create](/release/create) — 创建 Release
- [cnb release update](/release/update) — 更新 Release
- [cnb release delete](/release/delete) — 删除 Release
- [cnb release latest](/release/latest) — 查看最新 Release
- [cnb release download](/release/download) — 下载 Release 附件
- [cnb release asset-upload](/release/asset-upload) — 上传附件到 Release
- [cnb release asset-delete](/release/asset-delete) — 删除单个附件
- [cnb release asset-stats](/release/asset-stats) — 统计 Release 附件大小
- [cnb release asset-clean](/release/asset-clean) — 清理 Release 附件

## 示例

```bash
# 列出 Release
$ cnb release list

# 查看最新 Release
$ cnb release latest

# 创建并上传附件
$ cnb release create -t v1.0.0 -n "v1.0.0"
$ cnb release asset-upload -t v1.0.0 -f ./dist/app.tar.gz

# 下载附件
$ cnb release download v1.0.0 app.tar.gz
```

## 另请参阅

- [cnb](/guide/cnb)
