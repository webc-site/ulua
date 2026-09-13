//! @interface-stub
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::kind_a_64::KindA64,
  functions::lower_impl::lower_impl_a_64,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_options::AssemblyOptions,
    ir_builder::IrBuilder, ir_lowering_a_64::IrLoweringA64, ir_reg_alloc_a_64::IrRegAllocA64,
    ir_value_location_tracking::IrValueLocationTracking, lowering_stats::LoweringStats,
    module_helpers::ModuleHelpers, register_a_64::RegisterA64,
  },
};

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
    build: build as *mut AssemblyBuilderA64,
    helpers: helpers as *mut ModuleHelpers,
    function: &mut ir.function as *mut _,
    stats,
    regs,
    value_tracker,
    interrupt_handlers: Vec::new(),
    exit_handlers: Vec::new(),
    exit_handler_map: DenseHashMap::new(!0u32),
    exit_sync_alloc_token: 0,
    exit_sync_inst_idx: IrLoweringA64::K_INVALID_INST_IDX,
    error: false,
  };
  lowering.ir_lowering_a_64_ir_lowering_a_64();

  let bytecodeid = unsafe {
    if proto.is_null() {
      0
    } else {
      (*proto).bytecodeid
    }
  };

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
