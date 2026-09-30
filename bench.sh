#!/usr/bin/env bash

set -e
DIR=$(realpath "$0") && DIR=${DIR%/*}
cd "$DIR"

# 引擎后端互斥三选一（mlua ffi 扁平 re-export，单一二进制只能共链一种 C Lua），
# 故 exec 组按后端分三次跑，再以 --append 把部分结果并集合并进同一 results.json；
# compile/analysis 组只用 ulua 侧引擎（engine-luau 后端顺带跑 mlua/luau 编译对照）。
RUNNER="cargo run --manifest-path benchmarks/runner/Cargo.toml --release"

echo "=== 1/7 exec 组 · 后端 engine-luau (ulua ×2 + Luau C++ ×2) ==="
$RUNNER --features engine-luau -- --group=exec "$@"

echo "=== 2/7 exec 组 · 后端 engine-luajit (LuaJIT ×2, 只补测 LuaJIT 列) ==="
$RUNNER --features engine-luajit -- --group=exec --append --engines mlua/luajit,mlua/luajit-interp "$@"

echo "=== 3/7 exec 组 · 后端 engine-lua54 (PUC Lua 5.4, 只补测 Lua 5.4 列) ==="
$RUNNER --features engine-lua54 -- --group=exec --append --engines mlua/lua5.4 "$@"

echo "=== 4/7 编译吞吐组 (parse / parse+compile, ulua vs Luau C++) ==="
$RUNNER --features engine-luau -- --group=compile

echo "=== 5/7 类型检查组 (ulua-analysis 前端 strict 模式) ==="
$RUNNER --features engine-luau -- --group=analysis

echo "=== 6/7 转换数据为前端模块 (website/src/lib/benchData.js) ==="
if command -v bun >/dev/null 2>&1; then
  bun website/scripts/benchConvert.js
  echo "=== 7/7 生成离线矢量图表 (benchmarks/*.svg) ==="
  bun website/scripts/benchSvg.js
  if [ -f hook/svg.js ]; then
    bun hook/svg.js benchmarks/benchmark*.svg
  fi
else
  echo "提示: 未检测到 bun 命令，跳过 website 数据转换与 SVG 生成"
fi

echo "✓ 全集性能对比评测（7 引擎 × 16 用例 × exec/compile/analysis 三组）、数据转换与 SVG 矢量图生成完毕！"
