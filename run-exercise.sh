#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "用法: $0 exercises/<章节>/<练习>"
    exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exercise_dir="$repo_root/$1"
manifest="$exercise_dir/Cargo.toml"

if [[ ! -f "$manifest" ]]; then
    echo "找不到练习: $1"
    exit 1
fi

cargo test --manifest-path "$manifest"
