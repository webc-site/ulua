use core::mem::offset_of;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{call_vm_helper::call_vm_helper, luau_reg_address::luau_reg_address},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_data::K_INVALID_INST_IDX, ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64, native_context::NativeContext,
  },
};

pub fn call_length_helper(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  rb: i32,
) {
  // lua_v_dolen(L, ra, rb)
  call_vm_helper(
    regs,
    build,
    K_INVALID_INST_IDX,
    &[
      (SizeX64::Qword, luau_reg_address(ra), IrOp::new()),
      (SizeX64::Qword, luau_reg_address(rb), IrOp::new()),
    ],
    offset_of!(NativeContext, lua_v_dolen) as i32,
    true,
  );
}
