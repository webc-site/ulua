use core::mem::offset_of;

use ulua_vm::enums::tms::TMS;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{call_vm_helper::call_vm_helper, luau_reg_address::luau_reg_address},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_data::K_INVALID_INST_IDX, ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64, native_context::NativeContext, operand_x_64::OperandX64,
  },
};

pub fn call_arith_helper(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  b: OperandX64,
  c: OperandX64,
  tm: TMS,
) {
  // 按元方法标签选择 doarith 分派槽位
  let off = match tm {
    TMS::TmAdd => offset_of!(NativeContext, lua_v_doarithadd),
    TMS::TmSub => offset_of!(NativeContext, lua_v_doarithsub),
    TMS::TmMul => offset_of!(NativeContext, lua_v_doarithmul),
    TMS::TmDiv => offset_of!(NativeContext, lua_v_doarithdiv),
    TMS::TmIDiv => offset_of!(NativeContext, lua_v_doarithidiv),
    TMS::TmMod => offset_of!(NativeContext, lua_v_doarithmod),
    TMS::TmPow => offset_of!(NativeContext, lua_v_doarithpow),
    TMS::TmUnm => offset_of!(NativeContext, lua_v_doarithunm),
    _ => {
      CODEGEN_ASSERT!(false); // "Invalid doarith helper operation tag"
      0
    }
  } as i32;

  // lua_v_doarith*(L, ra, b, c)
  call_vm_helper(
    regs,
    build,
    K_INVALID_INST_IDX,
    &[
      (SizeX64::Qword, luau_reg_address(ra), IrOp::new()),
      (SizeX64::Qword, b, IrOp::new()),
      (SizeX64::Qword, c, IrOp::new()),
    ],
    off,
    true,
  );
}
