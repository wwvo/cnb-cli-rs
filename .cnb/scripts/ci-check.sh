#!/bin/bash

set -euo pipefail

action="${1:-all}"
target="${CARGO_BUILD_TARGET:-x86_64-unknown-linux-gnu}"

run_with_privilege() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
    return
  fi

  if command -v sudo >/dev/null 2>&1; then
    sudo "$@"
    return
  fi

  echo "缺少 root 权限，无法自动安装 Linux 系统依赖: $*" >&2
  exit 1
}

ensure_linux_system_deps() {
  if [ "$(uname -s)" != "Linux" ]; then
    return
  fi

  if command -v pkg-config >/dev/null 2>&1 && pkg-config --exists dbus-1; then
    return
  fi

  if ! command -v apt-get >/dev/null 2>&1; then
    echo "当前 Linux 环境未提供 apt-get，无法自动安装 dbus-1 开发包" >&2
    exit 1
  fi

  echo "缺少 Linux 系统依赖，正在安装 pkg-config 和 libdbus-1-dev..." >&2
  run_with_privilege env DEBIAN_FRONTEND=noninteractive apt-get update
  run_with_privilege env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    pkg-config \
    libdbus-1-dev
}

ensure_rust_component() {
  local component="${1}"
  local cargo_subcommand="${2}"

  if cargo "${cargo_subcommand}" --version >/dev/null 2>&1; then
    return
  fi

  local toolchain
  toolchain="$(rustup show active-toolchain | awk '{print $1}')"

  echo "缺少 Rust 组件 ${component}，正在为 ${toolchain} 安装..." >&2
  rustup component add "${component}" --toolchain "${toolchain}"
}

ensure_cargo_tool() {
  local tool="${1}"
  local subcommand="${2}"

  if cargo "${subcommand}" --version >/dev/null 2>&1; then
    return
  fi

  echo "缺少 ${tool}，正在安装..." >&2
  cargo install "${tool}" --locked
}

run_fmt() {
  ensure_rust_component rustfmt fmt
  cargo fmt --all --check
}

run_check() {
  ensure_linux_system_deps
  cargo check --workspace --target "${target}"
}

run_clippy() {
  ensure_linux_system_deps
  ensure_rust_component clippy clippy
  cargo clippy --workspace --all-targets --target "${target}" -- -W clippy::all -W clippy::pedantic
}

run_test() {
  ensure_linux_system_deps
  cargo test --workspace --target "${target}"
}

run_deny() {
  ensure_cargo_tool cargo-deny deny
  cargo deny check
}

case "${action}" in
  fmt)
    run_fmt
    ;;
  check)
    run_check
    ;;
  clippy)
    run_clippy
    ;;
  test)
    run_test
    ;;
  deny)
    run_deny
    ;;
  all)
    run_fmt
    run_check
    run_clippy
    run_test
    run_deny
    ;;
  *)
    echo "未知动作: ${action}" >&2
    echo "用法: ci-check.sh [fmt|check|clippy|test|deny|all]" >&2
    exit 1
    ;;
esac
