#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ ! -d "cpp" ]; then
  git clone --depth=1 https://github.com/webc-fork/luau.git cpp
fi

# 注入 fork 之前的上游源
git -C cpp remote add upstream https://github.com/luau-lang/luau.git 2>/dev/null || true

if [ ! -d "sh" ]; then
  ln -s "$HOME/.local/share/cargo_sh" sh
fi
