# Ulua Code Gen

Native A64/X64 JIT code generation for compiled Luau bytecode.

### Add to your project

```toml
[dependencies]
ulua-code-gen = "0.1.0"
```

## Links

- Docs: https://docs.rs/ulua-code-gen
- Repository: https://github.com/webc-site/ulua

## 已知未决：部分模块 native 入口被 gate 全量弹回（2026-10 调查留档）

bench 实测（M2 Max，exec 组 ulua-jit 列）：`fannkuch`/`microbig_gettable` 等 gate 放行
（`ecb.enter` 返 0，全程 native），但 `nbody`/`oop` 及极简复现 `/tmp/p1.lua`
（`local n=1e6; local function f() ... end; return f()`）**每次调用 gate 返 1**、93%+ 样本落
`tier_cold` 解释器——JIT 等价于关闭。已排除：编译失败（`compile_internal` 无
proto_failures）、fastflag 默认值（与 cpp 一致 1'048'576）、编译耗时污染（惰性编译，
`load=0`）、IR 语义（dump 核对守卫正确）。

已取证：
- p1 生成码 disasm 正确（常量/tag 检查/fadd 环与 IR 一致），但 RETURN 尾部
  `b`（跳 module helper）位移可疑（落回本 proto 内 +0x10）。
- 每次 gate 拒绝后 `ci->savedpc = code+20508`（越界垃圾；p1 sizecode 仅 8）——
  说明 native 入口跑了野分支或 exit veneer 写坏 savedpc。
- p1 的入口块缺 `FALLBACK_PREPVARARGS`（对照 p4 裸循环用例有、且 p4 放行），
  差异来自编译器把 `f` 内联进 main；是否因果未证。
- 复现：`exec_probe`（已在调查后删除，按 benchmarks/runner/src/bin 重写即可，
  Lua::new + enable_jit + load(src).into_function + call）+ `samply record`。

下一步建议：dump p1 与 p4 的 entry offsets 全表与 `ir.function.entry_location`，
核对 `create_native_proto_exec_data` 的 `entry_offset_or_address`（offset vs address
语义）与 gate `br X2` 目标基址；修复预期直接解锁 nbody（74ms→≈15ms 量级）、oop、
nbody 家族与官方 CodeGen 同款盲区（见表访问 IC 调研：动态键内联 / 两层 __index +
mini-PIC / megamorphic stub cache）。
