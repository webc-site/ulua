//! @interface-stub
use ulua_vm::records::proto::Proto;

use crate::{
  functions::{
    lower_impl::lower_impl_x_64,
    optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, assembly_options::AssemblyOptions,
    ir_builder::IrBuilder, ir_lowering_x_64::IrLoweringX64, lowering_stats::LoweringStats,
    module_helpers::ModuleHelpers,
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
    if proto.is_null() {
      0
    } else {
      (*proto).bytecodeid
    }
  };

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
