#!/bin/bash
# 校验发布 tag 格式，并确保其与 workspace 版本一致。
set -euo pipefail

TAG_NAME="${CNB_BRANCH:-}"

if [[ -z "${TAG_NAME}" ]]; then
  echo "错误: 未检测到发布标签"
  exit 1
fi

if [[ ! "${TAG_NAME}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-(alpha|beta)(\.[0-9]+)?)?$ ]]; then
  echo "错误: 标签 ${TAG_NAME} 不符合预期格式（例如 v1.2.3 或 v1.2.3-alpha.1）"
  exit 1
fi

TAG_VERSION="${TAG_NAME#v}"
WORKSPACE_VERSION="$(
  awk '
    /^\[workspace\.package\]/ { in_section=1; next }
    /^\[/ { in_section=0 }
    in_section && /^version[[:space:]]*=/ {
      line = $0
      sub(/^[^"]*"/, "", line)
      sub(/".*$/, "", line)
      print line
      exit
    }
  ' Cargo.toml
)"

if [[ -z "${WORKSPACE_VERSION}" ]]; then
  echo "错误: 无法从 Cargo.toml 读取 [workspace.package] version"
  exit 1
fi

if [[ "${TAG_VERSION}" != "${WORKSPACE_VERSION}" ]]; then
  echo "错误: 标签版本 ${TAG_VERSION} 与 Cargo.toml 版本 ${WORKSPACE_VERSION} 不一致"
  exit 1
fi

echo "版本校验通过: ${TAG_NAME} -> ${WORKSPACE_VERSION}"
