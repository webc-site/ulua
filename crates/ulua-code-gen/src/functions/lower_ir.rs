use core::ptr::{self, NonNull};

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::kind_a_64::KindA64,
  functions::{
    lower_impl::{lower_impl_a_64, lower_impl_x_64},
    optimize_memory_operands::optimize_memory_operands,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, ir_data::K_INVALID_INST_IDX,
    ir_lowering_a_64::IrLoweringA64, ir_lowering_x_64::IrLoweringX64,
    ir_reg_alloc_a_64::IrRegAllocA64, ir_value_location_tracking::IrValueLocationTracking,
    lowering_stats::LoweringStats, module::ModuleHelpers, register_a_64::RegisterA64,
  },
};

/// X64 IR→机器码末步：内存操作数优化 + 装配 `IrLoweringX64` 后移交 [`lower_impl_x_64`]。
pub(crate) fn lower_ir_x_64(
  build: &mut AssemblyBuilderX64,
  ir: &mut IrBuilder,
  sorted_blocks: &[u32],
  helpers: &mut ModuleHelpers,
  proto: Option<&Proto>,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
) -> bool {
  optimize_memory_operands(&mut ir.function);

  let mut lowering = IrLoweringX64::new(build, helpers, &mut ir.function, stats.map(NonNull::from));
  lowering.reset_restore_callback();

  // 显式判空短路：None 走常量分支（原 null 路径逐字等价），Some 只读 `bytecodeid`。
  let bytecodeid = proto.map_or(0, |proto| proto.bytecodeid);

  lower_impl_x_64(
    build,
    &mut lowering,
    &mut ir.function,
    sorted_blocks,
    bytecodeid,
    &options,
  )
}

/// A64 IR→机器码末步：寄存器区间装配 + 装配 `IrLoweringA64` 后移交 [`lower_impl_a_64`]。
pub(crate) fn lower_ir_a_64(
  build: &mut AssemblyBuilderA64,
  ir: &mut IrBuilder,
  sorted_blocks: &[u32],
  helpers: &mut ModuleHelpers,
  proto: Option<&Proto>,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
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

  let stats_handle = stats.map(NonNull::from);
  let regs = IrRegAllocA64::new(build, &mut ir.function, stats_handle, &reg_ranges);
  let value_tracker = IrValueLocationTracking::new(&mut ir.function);

  // lowering 内的 `build`/`helpers`/`function` 裸指针由本作用域存活引用以
  // `ptr::from_mut` 接线，比 `lowering` 长寿且指向合法对象；`stats` 可为 null，
  // 消费侧按 C++ 语义守卫。
  let mut lowering = IrLoweringA64 {
    build: ptr::from_mut(build),
    helpers: ptr::from_mut(helpers),
    function: ptr::from_mut(&mut ir.function),
    stats: stats_handle,
    regs,
    value_tracker,
    interrupt_handlers: Vec::new(),
    exit_handlers: Vec::new(),
    exit_handler_map: DenseHashMap::new(!0u32),
    exit_sync_alloc_token: 0,
    exit_sync_inst_idx: K_INVALID_INST_IDX,
    error: false,
  };
  lowering.setup_restore_callback();

  // 显式判空短路：None 走常量分支（原 null 路径逐字等价），Some 只读 `bytecodeid`。
  let bytecodeid = proto.map_or(0, |proto| proto.bytecodeid);

  lower_impl_a_64(
    build,
    &mut lowering,
    &mut ir.function,
    sorted_blocks,
    bytecodeid,
    &options,
  )
}
