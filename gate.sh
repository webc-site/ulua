#!/usr/bin/env bash
# 在指定 worktree 里跑全量门（clippy + test.sh），不动主目录工作区。
# 用法：./gate.sh /tmp/fork/gate-x   （worktree 需已 checkout 到待测提交）
# 说明：仓库根的 `sh` 是指向 ~/.local/share/cargo_sh 的符号链接且未纳入版本控制，
# 所以新建的 worktree 里没有 ./sh/clippy.sh——本脚本补链后再跑。
set -uo pipefail
wt=${1:?用法: ./gate.sh <worktree 路径>}
[ -e "$wt/.git" ] || { echo "不是 git worktree: $wt" >&2; exit 2; }
[ -e "$wt/sh" ] || ln -s /Users/z/.local/share/cargo_sh "$wt/sh"
cd "$wt" || exit 2
./sh/clippy.sh > /tmp/gate_clippy.log 2>&1
echo "clippy=$?"
./test.sh > /tmp/gate_test.log 2>&1
echo "test=$?"
rg -c '^(warning|error)' /tmp/gate_clippy.log
grep -E 'tests run:' /tmp/gate_test.log | tail -3
