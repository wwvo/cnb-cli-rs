#!/bin/bash

set -euo pipefail

required_vars=(
  GITHUB_SYNC_TARGET_URL
  GITHUB_SYNC_USERNAME
  GITHUB_SYNC_TOKEN
)

missing_vars=()

for var_name in "${required_vars[@]}"; do
  if [[ -z "${!var_name:-}" ]]; then
    missing_vars+=("${var_name}")
  fi
done

if [[ "${#missing_vars[@]}" -gt 0 ]]; then
  echo "错误: 缺少 GitHub 镜像同步配置: ${missing_vars[*]}" >&2
  echo "请通过 imports 或仓库环境变量提供这些变量" >&2
  exit 1
fi

echo "GitHub 镜像同步配置检查通过"
