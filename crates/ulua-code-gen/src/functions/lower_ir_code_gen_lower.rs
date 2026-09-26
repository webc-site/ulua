use core::ptr;

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::kind_a_64::KindA64,
  functions::{
    lower_impl::{lower_impl_a_64, lower_impl_x_64},
    optimize_memory_operands_x_64_optimize_final_x_64::optimize_memory_operands_x_64,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, ir_data::K_INVALID_INST_IDX,
    ir_lowering_a_64::IrLoweringA64, ir_lowering_x_64::IrLoweringX64,
    ir_reg_alloc_a_64::IrRegAllocA64, ir_value_location_tracking::IrValueLocationTracking,
    lowering_stats::LoweringStats, module_helpers::ModuleHelpers, register_a_64::RegisterA64,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats(
  build: &mut AssemblyBuilderX64,
  ir: &mut IrBuilder,
  sorted_blocks: &[u32],
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> bool {
  optimize_memory_operands_x_64(&mut ir.function);

  let mut lowering =
    IrLoweringX64::ir_lowering_x_64_ir_lowering_x_64(build, helpers, &mut ir.function, stats);
  lowering.reset_restore_callback();

  let bytecodeid = unsafe {
    // Safety: 显式判空——null 走常量分支；否则 `proto` 依契约指向存活 `Proto`，仅读 `bytecodeid` 字段。
    if proto.is_null() {
      0
    } else {
      (*proto).bytecodeid
    }
  };

  // Safety: `lower_impl_x_64` 的裸指针入参在此合法：`build`/`ir.function` 为本作用域存活引用的
  // 独占借用；`lowering` 持有的 `stats`（可 null，callee 按 C++ 语义守卫）与入参一致；
  // `sorted_blocks`/`options` 为调用方提供的合法借用。透传被调 `# Safety` 契约。
  unsafe {
    lower_impl_x_64(
      build,
      &mut lowering,
      &mut ir.function,
      sorted_blocks,
      bytecodeid,
      &options,
    )
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_ir_a_64_assembly_builder_a_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats(
  build: &mut AssemblyBuilderA64,
  ir: &mut IrBuilder,
  sorted_blocks: &[u32],
  helpers: &mut ModuleHelpers,
  proto: *mut Proto,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> bool {
  let reg_ranges = [
    (
      RegisterA64 {
        bits: KindA64::X as u8 | (0u8 << RegisterA64::INDEX_SHIFT),
      },
      RegisterA64 {
        bits: KindA64::X as u8 | (15u8 << RegisterA64::INDEX_SHIFT),
      },
    ),
    (
      RegisterA64 {
        bits: KindA64::X as u8 | (16u8 << RegisterA64::INDEX_SHIFT),
      },
      RegisterA64 {
        bits: KindA64::X as u8 | (17u8 << RegisterA64::INDEX_SHIFT),
      },
    ),
    (
      RegisterA64 {
        bits: KindA64::Q as u8 | (0u8 << RegisterA64::INDEX_SHIFT),
      },
      RegisterA64 {
        bits: KindA64::Q as u8 | (7u8 << RegisterA64::INDEX_SHIFT),
      },
    ),
    (
      RegisterA64 {
        bits: KindA64::Q as u8 | (16u8 << RegisterA64::INDEX_SHIFT),
      },
      RegisterA64 {
        bits: KindA64::Q as u8 | (31u8 << RegisterA64::INDEX_SHIFT),
      },
    ),
  ];

  let regs =
    IrRegAllocA64::ir_reg_alloc_a_64_ir_reg_alloc_a_64(build, &mut ir.function, stats, &reg_ranges);
  let value_tracker = IrValueLocationTracking::new(&mut ir.function);

  let mut lowering = IrLoweringA64 {
    build: ptr::from_mut(build),
    helpers: ptr::from_mut(helpers),
    function: ptr::from_mut(&mut ir.function),
    stats,
    regs,
    value_tracker,
    interrupt_handlers: Vec::new(),
    exit_handlers: Vec::new(),
    exit_handler_map: DenseHashMap::new(!0u32),
    exit_sync_alloc_token: 0,
    exit_sync_inst_idx: K_INVALID_INST_IDX,
    error: false,
  };
  lowering.ir_lowering_a_64_ir_lowering_a_64();

  let bytecodeid = unsafe {
    // Safety: 显式判空——null 走常量分支；否则 `proto` 指向存活 `Proto`，仅读 `bytecodeid`。
    if proto.is_null() {
      0
    } else {
      (*proto).bytecodeid
    }
  };

  // Safety: `lowering` 内的 `build`/`helpers`/`function` 裸指针在上方由本作用域存活引用
  // （`build`、`helpers`、`&mut ir.function`）以 `ptr::from_mut` 洗白接线，比 `lowering` 长寿且指向合法
  // 对象；`stats` 可为 null，由 `lower_impl_a_64` 按 C++ 语义守卫。透传被调 `# Safety` 契约。
  unsafe {
    lower_impl_a_64(
      build,
      &mut lowering,
      &mut ir.function,
      sorted_blocks,
      bytecodeid,
      &options,
    )
  }
}
