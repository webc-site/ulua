// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
//! 反汇编双目标的公共断言块：对应 cpp `tests/Conformance.test.cpp:453-479`
//! （`runConformance` 尾部「Extra test for lowering on both platforms with assembly
//! generation」）。上游 `LargeModuleA64`（`:4964-4981`）只跑 A64 且只开
//! `includeAssembly`，故不复用本函数。

use core::ptr::null_mut;

use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    lowering_stats::LoweringStats,
  },
};
use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::assembly;

/// 把栈顶函数分别按 A64 / X64(SystemV) 反汇编一次，两条目标都要求
/// 有输出且 `regAllocErrors` / `loweringErrors` 为零。
///
/// `stats` 按上游一样跨两次调用累积复用（第二次断言因此同时覆盖两遍的累计值）。
///
/// `l` 须为存活 `LuaState` 且栈顶 `-1` 是 Lua 函数（cpp `getAssembly` 的 LUAU_ASSERT）
/// ——该前提在 [`assembly`] 门面的 Safety 契约里收口（review.md §2 最小边界）。
pub fn dump_assembly_both_targets(l: *mut LuaState, compilation_options: CompilationOptions) {
  let mut stats = LoweringStats {
    function_stats_flags: FunctionStatsFlags::FunctionStatsEnable as u32,
    ..Default::default()
  };

  let mut assembly_options = AssemblyOptions {
    target: Target::A64,
    compilation_options,
    output_binary: false,
    include_assembly: true,
    include_ir: true,
    include_outlined_code: true,
    include_ir_types: true,
    include_ir_prefix: Default::default(),
    include_use_info: Default::default(),
    include_cfg_info: Default::default(),
    include_reg_flow_info: Default::default(),
    annotator: None,
    // FFI: c-API 要求 NULL
    annotator_context: null_mut(),
  };

  let a64 = assembly(l, -1, assembly_options.clone(), Some(&mut stats));
  assert!(!a64.is_empty(), "A64 assembly dump is empty");
  assert_eq!(0, stats.reg_alloc_errors, "A64 regAllocErrors");
  assert_eq!(0, stats.lowering_errors, "A64 loweringErrors");

  assembly_options.target = Target::X64SystemV;
  let x64 = assembly(l, -1, assembly_options, Some(&mut stats));
  assert!(!x64.is_empty(), "X64 assembly dump is empty");
  assert_eq!(0, stats.reg_alloc_errors, "X64 regAllocErrors");
  assert_eq!(0, stats.lowering_errors, "X64 loweringErrors");
}
