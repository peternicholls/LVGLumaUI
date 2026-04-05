#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/lumaui-phase-check.sh [options]

Runs the standard LumaUI verification bundle for the active brownfield slice.

Options:
  --project PATH     Project to check with doctor/validate/build. Default: examples/minimal
  --skip-build       Skip the build command entirely
  --require-build    Require the build command to succeed
  -h, --help         Show this help text
EOF
}

project="examples/minimal"
build_mode="allow-gated"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --project)
      if [[ $# -lt 2 ]]; then
        echo "error: --project requires a path" >&2
        exit 2
      fi
      project="$2"
      shift 2
      ;;
    --skip-build)
      build_mode="skip"
      shift
      ;;
    --require-build)
      build_mode="require"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"

run_step() {
  local label="$1"
  shift

  echo "==> ${label}"
  (cd "${repo_root}" && "$@")
}

run_build_step() {
  local output

  echo "==> cargo run -p lumaui-cli -- build ${project}"
  set +e
  output="$(cd "${repo_root}" && cargo run -p lumaui-cli -- build "${project}" 2>&1)"
  local status=$?
  set -e

  printf '%s\n' "${output}"

  if [[ ${status} -eq 0 ]]; then
    echo "build check: passed"
    return 0
  fi

  if [[ "${build_mode}" == "allow-gated" ]] && grep -Fq "build is not available yet; parser and semantic lowering are still being implemented" <<<"${output}"; then
    echo "build check: gated as expected for the current slice"
    return 0
  fi

  echo "build check: failed" >&2
  return "${status}"
}

run_step "cargo test --workspace" cargo test --workspace
run_step "cargo run -p lumaui-cli -- doctor ${project}" cargo run -p lumaui-cli -- doctor "${project}"
run_step "cargo run -p lumaui-cli -- validate ${project}" cargo run -p lumaui-cli -- validate "${project}"

case "${build_mode}" in
  skip)
    echo "==> build check skipped"
    ;;
  allow-gated|require)
    run_build_step
    ;;
esac
