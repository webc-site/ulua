use ulua_code_gen::{
  functions::get_assembly::get_assembly,
  records::{assembly_options::AssemblyOptions, lowering_stats::LoweringStats},
};
use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  records::lua_state_guard::LuaStateGuard,
};

/// cpp `getCodegenAssembly` (CLI/src/Compile.cpp:114-129)
///
/// 返回机器码或 asm/IR 文本的字节（cpp 侧同一名 `std::string` 承载两者）。
/// 错误消息不再直写 stderr，而是追加进调用方按文件持有的 `stderr` 缓冲，
/// 由主线程按 `files` 原顺序统一输出（并行编译下保持与串行版逐字节一致）。
pub fn get_codegen_assembly(
  name: &str,
  bytecode: &[u8],
  options: AssemblyOptions,
  stats: &mut LoweringStats,
  stderr: &mut String,
) -> Vec<u8> {
  // cpp 侧 `name.c_str()` 由 `luau_load` 按 `strlen` 取值：这里直传 `&str`，
  // 内部 NUL 的截断规则已在 `luau_load` 一侧还原
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;
  // cpp Compile.cpp:114-129 未判空即调 `luau_load`（OOM 时对 null 解引用，UB）；
  // 这里按仓库内 Bytecode CLI `analyze_file` 的同款约定显式失败，
  // 同时让下方 unsafe 调用的指针契约得以成立。
  if l.is_null() {
    stderr.push_str("Error initializing Lua state\n");
    return Vec::new();
  }

  // Safety: `l` 非空由上方判空保证（存活期至 guard Drop，覆盖整次调用）；
  // `name`/`bytecode` 为调用帧存活的合法借用，`luau_load` 在调用内读完；
  // -1 仅在 load 成功后指向压入的原型。
  if unsafe { luau_load(l, name, bytecode, 0) == 0 } {
    // Safety: `l` 存活同上；-1 为 `luau_load` 成功压入的原型槽；
    // `stats` 为调用帧独占可变引用，仅被累加。
    unsafe { get_assembly(l, -1, options, stats) }
  } else {
    stderr.push_str(&format!("Error loading bytecode {name}\n"));
    Vec::new()
  }
}
