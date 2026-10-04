use core::mem::offset_of;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{call_vm_helper::call_vm_helper, emit::x_64::luau_reg_address},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    ir_data::K_INVALID_INST_IDX,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    native_context::NativeContext,
    operand_x_64::OperandX64,
  },
};

/// ulua_codegen_setmetatable(L, obj, mt) → i32（1=已赋值，0=前置不满足走 fallback）。
/// 本 fork 扩展：FASTCALL2(LBF_SETMETATABLE) 内联守卫后的赋值段直调。
pub fn call_set_metatable(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  obj: i32,
  mt: i32,
) {
  call_vm_helper(
    regs,
    build,
    K_INVALID_INST_IDX,
    &[
      (SizeX64::Qword, luau_reg_address(obj), IrOp::new()),
      (SizeX64::Qword, luau_reg_address(mt), IrOp::new()),
    ],
    offset_of!(NativeContext, setmetatable_checked) as i32,
    true,
  );
}
