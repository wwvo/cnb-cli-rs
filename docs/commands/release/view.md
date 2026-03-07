# cnb release view

```
cnb release view <TAG> [options]
```

查看指定 Release 的详细信息，包括基本信息、描述内容和附件列表。

## 参数

`TAG`
: Tag 名称（必填）

## 选项

`-w, --web`
: 在浏览器中打开 Release 页面

## 输出示例

```
v1.2.0 — v1.2.0
状态: Latest
作者: zhangsan
发布时间: 2025-01-15 10:30

## 更新内容
- 新增仓库管理功能
- 修复若干 Bug

附件:
NAME                          SIZE        DOWNLOADS
app-linux-amd64.tar.gz        15.2 MB     128
app-darwin-arm64.tar.gz       14.8 MB     96
checksums.txt                 256 B       45
```

## 示例

```bash
# 查看 Release 详情
$ cnb release view v1.2.0

# 在浏览器中打开
$ cnb release view v1.2.0 --web

# JSON 输出
$ cnb release view v1.2.0 --json
```

## 另请参阅

- [cnb release](/release/)
- [cnb release latest](/release/latest)
