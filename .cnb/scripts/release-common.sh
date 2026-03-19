#!/bin/bash

set -euo pipefail

cnb_log() {
  echo "[release] $*"
}

cnb_err() {
  echo "错误: $*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || cnb_err "缺少命令: $1"
}

api_endpoint() {
  printf '%s' "${CNB_API_ENDPOINT:-https://api.cnb.cool}"
}

repo_slug() {
  if [[ -n "${CNB_REPO_SLUG_LOWERCASE:-}" ]]; then
    printf '%s' "${CNB_REPO_SLUG_LOWERCASE}"
    return
  fi

  local remote
  remote="$(git remote get-url origin 2>/dev/null || true)"
  [[ -n "${remote}" ]] || cnb_err "无法推导仓库路径，请设置 CNB_REPO_SLUG_LOWERCASE"

  printf '%s' "${remote}" |
    sed -E 's#^https?://[^/]+/##; s#\.git$##'
}

extract_workspace_version() {
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
  ' "$@"
}

workspace_version() {
  extract_workspace_version Cargo.toml
}

workspace_version_at_ref() {
  local ref="$1"

  git cat-file -e "${ref}:Cargo.toml" 2>/dev/null || return 1
  git show "${ref}:Cargo.toml" | extract_workspace_version
}

set_workspace_version() {
  local new_version="$1"
  local temp_file
  temp_file="$(mktemp)"

  awk -v new_version="${new_version}" '
    /^\[workspace\.package\]/ { in_section=1; print; next }
    in_section && /^\[/ { in_section=0 }
    in_section && /^version[[:space:]]*=/ {
      print "version = \"" new_version "\""
      next
    }
    { print }
  ' Cargo.toml > "${temp_file}"

  mv "${temp_file}" Cargo.toml
}

set_installer_default_version() {
  local new_version="$1"
  local version_tag
  version_tag="$(version_tag "${new_version}")"

  set_shell_installer_default_version "${version_tag}"
  set_powershell_installer_default_version "${version_tag}"
}

set_shell_installer_default_version() {
  local version_tag="$1"
  local file="scripts/install.sh"
  local temp_file
  temp_file="$(mktemp)"

  awk -v version_tag="${version_tag}" '
    /^default_version=/ {
      print "default_version=\"" version_tag "\""
      replaced=1
      next
    }
    { print }
    END {
      if (!replaced) {
        exit 1
      }
    }
  ' "${file}" > "${temp_file}" || cnb_err "无法更新 ${file} 中的默认版本号"

  mv "${temp_file}" "${file}"
}

set_powershell_installer_default_version() {
  local version_tag="$1"
  local file="scripts/install.ps1"
  local temp_file
  temp_file="$(mktemp)"

  awk -v version_tag="${version_tag}" '
    /^\$DefaultVersion = / {
      print "$DefaultVersion = \"" version_tag "\""
      replaced=1
      next
    }
    { print }
    END {
      if (!replaced) {
        exit 1
      }
    }
  ' "${file}" > "${temp_file}" || cnb_err "无法更新 ${file} 中的默认版本号"

  mv "${temp_file}" "${file}"
}

version_tag() {
  printf 'v%s' "$1"
}

version_range() {
  local version="$1"
  local tag
  tag="$(version_tag "${version}")"

  if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null 2>&1; then
    printf '%s..HEAD' "${tag}"
  fi
}

commit_count_in_range() {
  local range="${1:-}"

  if [[ -n "${range}" ]]; then
    git rev-list --count "${range}"
  else
    git rev-list --count HEAD
  fi
}

detect_bump_level() {
  local range="${1:-}"
  local body_log_args=(--format=%B)
  local subject_log_args=(--format=%s)

  if [[ -n "${range}" ]]; then
    body_log_args+=("${range}")
    subject_log_args+=("${range}")
  fi

  local commit_bodies commit_subjects
  commit_bodies="$(git log "${body_log_args[@]}")"
  commit_subjects="$(git log "${subject_log_args[@]}")"

  # 提交主题允许带 emoji / 装饰前缀，例如 `✨ feat(dist): ...`
  if grep -Eq '(^|[[:space:]])BREAKING CHANGE:|^[^[:alnum:]]*[[:alpha:]]+(\([^)]*\))?!:' <<<"${commit_bodies}" ||
    grep -Eq '^[^[:alnum:]]*[[:alpha:]]+(\([^)]*\))?!:' <<<"${commit_subjects}"; then
    echo "major"
  elif grep -Eq '^[^[:alnum:]]*feat(\(|:|!)' <<<"${commit_subjects}"; then
    echo "minor"
  else
    echo "patch"
  fi
}

bump_version() {
  local version="$1"
  local level="$2"
  local major minor patch

  IFS='.' read -r major minor patch <<<"${version}"

  case "${level}" in
    major)
      if (( major == 0 )); then
        minor=$((minor + 1))
        patch=0
      else
        major=$((major + 1))
        minor=0
        patch=0
      fi
      ;;
    minor)
      minor=$((minor + 1))
      patch=0
      ;;
    patch)
      patch=$((patch + 1))
      ;;
    *)
      cnb_err "未知版本升级级别: ${level}"
      ;;
  esac

  printf '%s.%s.%s' "${major}" "${minor}" "${patch}"
}

json_escape() {
  printf '%s' "$1" | sed ':a;N;$!ba;s/\\/\\\\/g;s/"/\\"/g;s/\r//g;s/\n/\\n/g'
}

current_head_sha() {
  git rev-parse HEAD
}

require_cnb_token() {
  [[ -n "${CNB_TOKEN:-}" ]] || cnb_err "流水线缺少 CNB_TOKEN，无法调用 CNB OpenAPI"
}

create_pull_request() {
  local base_branch="$1"
  local head_branch="$2"
  local title="$3"
  local body="$4"
  local repo
  local payload_file

  require_cmd curl
  require_cnb_token

  repo="$(repo_slug)"
  payload_file="$(mktemp)"

  cat > "${payload_file}" <<EOF
{
  "base": "$(json_escape "${base_branch}")",
  "head": "$(json_escape "${head_branch}")",
  "head_repo": "$(json_escape "${repo}")",
  "title": "$(json_escape "${title}")",
  "body": "$(json_escape "${body}")"
}
EOF

  curl -fsS \
    -X POST \
    "$(api_endpoint)/${repo}/-/pulls" \
    -H "Accept: application/vnd.cnb.api+json" \
    -H "Authorization: Bearer ${CNB_TOKEN}" \
    -H "Content-Type: application/json" \
    --data-binary @"${payload_file}"

  rm -f "${payload_file}"
}

create_tag() {
  local tag_name="$1"
  local target="$2"
  local message="$3"
  local repo
  local payload_file

  require_cmd curl
  require_cnb_token

  repo="$(repo_slug)"
  payload_file="$(mktemp)"

  cat > "${payload_file}" <<EOF
{
  "name": "$(json_escape "${tag_name}")",
  "target": "$(json_escape "${target}")",
  "message": "$(json_escape "${message}")"
}
EOF

  curl -fsS \
    -X POST \
    "$(api_endpoint)/${repo}/-/git/tags" \
    -H "Accept: application/vnd.cnb.api+json" \
    -H "Authorization: Bearer ${CNB_TOKEN}" \
    -H "Content-Type: application/json" \
    --data-binary @"${payload_file}"

  rm -f "${payload_file}"
}
