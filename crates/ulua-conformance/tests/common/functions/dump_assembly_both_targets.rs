//! 反汇编双目标的公共断言块：对应 cpp `tests/Conformance.test.cpp:453-479`
//! （`runConformance` 尾部「Extra test for lowering on both platforms with assembly
//! generation」）。上游 `LargeModuleA64`（`:4964-4981`）只跑 A64 且只开
//! `includeAssembly`，故不复用本函数。

use core::ptr::null_mut;

use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  functions::get_assembly::get_assembly,
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    lowering_stats::LoweringStats,
  },
};
use ulua_vm::records::lua_state::LuaState;

/// 把栈顶函数分别按 A64 / X64(SystemV) 反汇编一次，两条目标都要求
/// 有输出且 `regAllocErrors` / `loweringErrors` 为零。
///
/// `stats` 按上游一样跨两次调用累积复用（第二次断言因此同时覆盖两遍的累计值）。
///
/// `l` 须为存活 `LuaState` 且栈顶 `-1` 是 Lua 函数（cpp `getAssembly` 的 LUAU_ASSERT）
/// ——该前提在两个 `get_assembly` 收口点各自核对（review.md §2 最小边界）。
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

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`stats` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let a64 = unsafe { get_assembly(l, -1, assembly_options.clone(), &mut stats) };
  assert!(!a64.is_empty(), "A64 assembly dump is empty");
  assert_eq!(0, stats.reg_alloc_errors, "A64 regAllocErrors");
  assert_eq!(0, stats.lowering_errors, "A64 loweringErrors");

  assembly_options.target = Target::X64SystemV;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`stats` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let x64 = unsafe { get_assembly(l, -1, assembly_options, &mut stats) };
  assert!(!x64.is_empty(), "X64 assembly dump is empty");
  assert_eq!(0, stats.reg_alloc_errors, "X64 regAllocErrors");
  assert_eq!(0, stats.lowering_errors, "X64 loweringErrors");
}
